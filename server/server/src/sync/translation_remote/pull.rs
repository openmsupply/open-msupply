use log::{info, warn};
use repository::{
    schema::{
        InvoiceLineRow, InvoiceRow, NameStoreJoinRow, NumberRow, RemoteSyncBufferRow, StockLineRow,
        StocktakeLineRow, StocktakeRow,
    },
    InvoiceLineRowRepository, InvoiceRepository, NameStoreJoinRepository, NumberRowRepository,
    RepositoryError, StockLineRowRepository, StocktakeLineRowRepository, StocktakeRowRepository,
    StorageConnection, TransactionError,
};

use crate::sync::{
    translation_remote::{
        name_store_join::NameStoreJoinTranslation, number::NumberTranslation,
        shipment::ShipmentTranslation, shipment_line::ShipmentLineTranslation,
        stock_line::StockLineTranslation, stocktake::StocktakeTranslation,
        stocktake_line::StocktakeLineTranslation,
    },
    SyncImportError, SyncTranslationError,
};

#[derive(Debug, Clone, PartialEq)]
pub enum IntegrationUpsertRecord {
    Number(NumberRow),
    StockLine(StockLineRow),
    NameStoreJoin(NameStoreJoinRow),
    Shipment(InvoiceRow),
    ShipmentLine(InvoiceLineRow),
    Stocktake(StocktakeRow),
    StocktakeLine(StocktakeLineRow),
}
#[derive(Debug, Clone, PartialEq)]
pub struct IntegrationRecord {
    pub upserts: Vec<IntegrationUpsertRecord>,
}

impl IntegrationRecord {
    pub fn from_upsert(record: IntegrationUpsertRecord) -> Self {
        IntegrationRecord {
            upserts: vec![record],
        }
    }
}

pub trait RemotePullTranslation {
    fn try_translate_pull(
        &self,
        connection: &StorageConnection,
        sync_record: &RemoteSyncBufferRow,
    ) -> Result<Option<IntegrationRecord>, SyncTranslationError>;
}

/// Imports sync records and writes them to the DB
/// If needed data records are translated to the local DB schema.
pub fn import_sync_pull_records(
    connection: &StorageConnection,
    records: &Vec<RemoteSyncBufferRow>,
) -> Result<(), SyncImportError> {
    let mut integration_records = IntegrationRecord {
        upserts: Vec::new(),
    };

    info!(
        "Translating {} remote sync buffer records...",
        records.len()
    );
    for record in records {
        do_translation(connection, &record, &mut integration_records)?;
    }
    info!("Succesfully translated remote sync buffer records");

    info!("Storing integration remote records...");
    store_integration_records(connection, &integration_records)?;
    info!("Successfully stored integration remote records");

    Ok(())
}

fn do_translation(
    connection: &StorageConnection,
    sync_record: &RemoteSyncBufferRow,
    records: &mut IntegrationRecord,
) -> Result<(), SyncTranslationError> {
    let translations: Vec<Box<dyn RemotePullTranslation>> = vec![
        Box::new(NumberTranslation {}),
        Box::new(StockLineTranslation {}),
        Box::new(NameStoreJoinTranslation {}),
        Box::new(ShipmentTranslation {}),
        Box::new(ShipmentLineTranslation {}),
        Box::new(StocktakeTranslation {}),
        Box::new(StocktakeLineTranslation {}),
    ];
    for translation in translations {
        if let Some(mut result) = translation.try_translate_pull(connection, sync_record)? {
            records.upserts.append(&mut result.upserts);
            return Ok(());
        }
    }
    warn!("Unhandled remote pull record: {:?}", sync_record);
    Ok(())
}

fn integrate_record(
    record: &IntegrationUpsertRecord,
    con: &StorageConnection,
) -> Result<(), RepositoryError> {
    match &record {
        IntegrationUpsertRecord::Number(record) => NumberRowRepository::new(con).upsert_one(record),
        IntegrationUpsertRecord::StockLine(record) => {
            StockLineRowRepository::new(con).upsert_one(record)
        }
        IntegrationUpsertRecord::NameStoreJoin(record) => {
            NameStoreJoinRepository::new(con).upsert_one(record)
        }
        IntegrationUpsertRecord::Shipment(record) => InvoiceRepository::new(con).upsert_one(record),
        IntegrationUpsertRecord::ShipmentLine(record) => {
            InvoiceLineRowRepository::new(con).upsert_one(record)
        }
        IntegrationUpsertRecord::Stocktake(record) => {
            StocktakeRowRepository::new(con).upsert_one(record)
        }
        IntegrationUpsertRecord::StocktakeLine(record) => {
            StocktakeLineRowRepository::new(con).upsert_one(record)
        }
    }
}

fn store_integration_records(
    connection: &StorageConnection,
    integration_records: &IntegrationRecord,
) -> Result<(), SyncImportError> {
    connection
        .transaction_sync(|con| {
            for record in &integration_records.upserts {
                // Integrate every record in a sub transaction. This is mainly for Postgres where the
                // whole transaction fails when there is a DB error (not a problem in sqlite).
                let sub_result =
                    con.transaction_sync_etc(|sub_tx| integrate_record(record, sub_tx), false);
                match sub_result {
                    Ok(_) => Ok(()),
                    Err(TransactionError::Inner(err @ RepositoryError::ForeignKeyViolation(_))) => {
                        warn!("Failed to import ({}): {:?}", err, record);
                        Ok(())
                    }
                    Err(err) => Err(SyncImportError::as_integration_error(
                        RepositoryError::from(err),
                        "",
                    )),
                }?;
            }
            Ok(())
        })
        .map_err(|error| match error {
            TransactionError::Transaction { msg, level } => SyncImportError::as_integration_error(
                RepositoryError::TransactionError { msg, level },
                "",
            ),
            TransactionError::Inner(e) => e,
        })
}

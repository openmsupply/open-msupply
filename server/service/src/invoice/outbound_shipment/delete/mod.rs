use domain::{invoice_line::InvoiceLine, EqualFilter, outbound_shipment::DeleteOutboundShipmentLine};
use repository::{
    InvoiceLineFilter, InvoiceLineRepository, InvoiceRepository, RepositoryError,
    StorageConnectionManager, TransactionError,
};

pub mod validate;

use validate::validate;

use crate::{invoice_line::{delete_outbound_shipment_line, DeleteOutboundShipmentLineError}, WithDBError};

pub fn delete_outbound_shipment(
    connection_manager: &StorageConnectionManager,
    id: String,
) -> Result<String, DeleteOutboundShipmentError> {
    let connection = connection_manager.connection()?;
    connection.transaction_sync(|connection| {
        validate(&id, &connection)?;

        // TODO https://github.com/openmsupply/remote-server/issues/839
        let lines = InvoiceLineRepository::new(&connection)
            .query_by_filter(InvoiceLineFilter::new().invoice_id(EqualFilter::equal_to(&id)))?;
        for line in lines {
            delete_outbound_shipment_line(
                connection_manager,
                DeleteOutboundShipmentLine {
                    id: line.id.clone(),
                    invoice_id: id.clone(),
                },
            )
            .map_err(|error| DeleteOutboundShipmentError::LineDeleteError {
                line_id: line.id,
                error,
            })?;
        }
        // End TODO

        InvoiceRepository::new(&connection).delete(&id)?;
        Ok(())
    })?;
    Ok(id)
}

pub enum DeleteOutboundShipmentError {
    InvoiceDoesNotExist,
    DatabaseError(RepositoryError),
    NotThisStoreInvoice,
    CannotEditFinalised,
    InvoiceLinesExists(Vec<InvoiceLine>),
    LineDeleteError {
        line_id: String,
        error: DeleteOutboundShipmentLineError,
    },
    NotAnOutboundShipment,
}

impl From<RepositoryError> for DeleteOutboundShipmentError {
    fn from(error: RepositoryError) -> Self {
        DeleteOutboundShipmentError::DatabaseError(error)
    }
}

impl From<TransactionError<DeleteOutboundShipmentError>> for DeleteOutboundShipmentError {
    fn from(error: TransactionError<DeleteOutboundShipmentError>) -> Self {
        match error {
            TransactionError::Transaction { msg, level } => {
                DeleteOutboundShipmentError::DatabaseError(RepositoryError::TransactionError {
                    msg,
                    level,
                })
            }
            TransactionError::Inner(e) => e,
        }
    }
}

impl<ERR> From<WithDBError<ERR>> for DeleteOutboundShipmentError
where
    ERR: Into<DeleteOutboundShipmentError>,
{
    fn from(result: WithDBError<ERR>) -> Self {
        match result {
            WithDBError::DatabaseError(error) => error.into(),
            WithDBError::Error(error) => error.into(),
        }
    }
}

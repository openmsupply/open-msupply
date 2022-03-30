use log::warn;
use repository::{
    schema::{ChangelogRow, ChangelogTableName, NameStoreJoinRow, RemoteSyncBufferRow},
    NameRepository, NameStoreJoinRepository, StorageConnection,
};

use serde::{Deserialize, Serialize};

use crate::sync::SyncTranslationError;

use super::{
    pull::{IntegrationRecord, IntegrationUpsertRecord, RemotePullTranslation},
    push::{to_push_translation_error, PushUpsertRecord, RemotePushUpsertTranslation},
    TRANSLATION_RECORD_NAME_STORE_JOIN,
};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct LegacyNameStoreJoinRow {
    pub ID: String,
    pub store_ID: String,
    pub name_ID: String,
    #[serde(rename = "om_name_is_customer")]
    pub name_is_customer: Option<bool>,
    #[serde(rename = "om_name_is_supplier")]
    pub name_is_supplier: Option<bool>,
}

pub struct NameStoreJoinTranslation {}
impl RemotePullTranslation for NameStoreJoinTranslation {
    fn try_translate_pull(
        &self,
        connection: &StorageConnection,
        sync_record: &RemoteSyncBufferRow,
    ) -> Result<Option<super::pull::IntegrationRecord>, SyncTranslationError> {
        let table_name = TRANSLATION_RECORD_NAME_STORE_JOIN;

        if sync_record.table_name != table_name {
            return Ok(None);
        }

        let data = serde_json::from_str::<LegacyNameStoreJoinRow>(&sync_record.data).map_err(
            |source| SyncTranslationError {
                table_name,
                source: source.into(),
                record: sync_record.data.clone(),
            },
        )?;

        let name = match NameRepository::new(connection)
            .find_one_by_id(&data.name_ID)
            .map_err(|source| SyncTranslationError {
                table_name,
                source: source.into(),
                record: sync_record.data.clone(),
            })? {
            Some(name) => name,
            None => {
                // TODO: support patients?
                warn!(
                    "Failed to get name \"{}\" for name_store_join \"{}\". Potentially the name refers to a patient but patients are currently not synced.",
                    data.name_ID, data.ID
                );
                return Ok(None);
            }
        };

        Ok(Some(IntegrationRecord::from_upsert(
            IntegrationUpsertRecord::NameStoreJoin(NameStoreJoinRow {
                id: data.ID,
                name_id: data.name_ID,
                store_id: data.store_ID,
                name_is_customer: data.name_is_customer.unwrap_or(name.is_customer),
                name_is_supplier: data.name_is_supplier.unwrap_or(name.is_supplier),
            }),
        )))
    }
}

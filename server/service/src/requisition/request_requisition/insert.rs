use crate::{
    number::next_number,
    requisition::{common::check_requisition_exists, query::get_requisition},
    service_provider::ServiceContext,
    validate::{check_other_party, CheckOtherPartyType, OtherPartyErrors},
};
use chrono::Utc;
use repository::{
    schema::{NumberRowType, RequisitionRow, RequisitionRowStatus, RequisitionRowType},
    RepositoryError, Requisition, RequisitionRowRepository, StorageConnection,
};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct InsertRequestRequisition {
    pub id: String,
    pub other_party_id: String,
    pub colour: Option<String>,
    pub their_reference: Option<String>,
    pub comment: Option<String>,
    pub max_months_of_stock: f64,
    pub min_months_of_stock: f64,
}

#[derive(Debug, PartialEq)]

pub enum InsertRequestRequisitionError {
    RequisitionAlreadyExists,
    // Name validation
    OtherPartyNotASupplier,
    OtherPartyDoesNotExist,
    OtherPartyNotVisible,
    OtherPartyIsNotAStore,
    // Internal
    NewlyCreatedRequisitionDoesNotExist,
    DatabaseError(RepositoryError),
}

type OutError = InsertRequestRequisitionError;

pub fn insert_request_requisition(
    ctx: &ServiceContext,
    store_id: &str,
    user_id: &str,
    input: InsertRequestRequisition,
) -> Result<Requisition, OutError> {
    let requisition = ctx
        .connection
        .transaction_sync(|connection| {
            validate(connection, store_id, &input)?;
            let new_requisition = generate(connection, store_id, user_id, input)?;
            RequisitionRowRepository::new(&connection).upsert_one(&new_requisition)?;

            get_requisition(ctx, None, &new_requisition.id)
                .map_err(|error| OutError::DatabaseError(error))?
                .ok_or(OutError::NewlyCreatedRequisitionDoesNotExist)
        })
        .map_err(|error| error.to_inner_error())?;
    Ok(requisition)
}

fn validate(
    connection: &StorageConnection,
    store_id: &str,
    input: &InsertRequestRequisition,
) -> Result<(), OutError> {
    if let Some(_) = check_requisition_exists(connection, &input.id)? {
        return Err(OutError::RequisitionAlreadyExists);
    }

    let other_party = check_other_party(
        connection,
        store_id,
        &input.other_party_id,
        CheckOtherPartyType::Supplier,
    )
    .map_err(|e| match e {
        OtherPartyErrors::OtherPartyDoesNotExist => OutError::OtherPartyDoesNotExist {},
        OtherPartyErrors::OtherPartyNotVisible => OutError::OtherPartyNotVisible,
        OtherPartyErrors::TypeMismatched => OutError::OtherPartyNotASupplier,
        OtherPartyErrors::DatabaseError(repository_error) => {
            OutError::DatabaseError(repository_error)
        }
    })?;

    other_party
        .store_id()
        .ok_or(OutError::OtherPartyIsNotAStore)?;

    Ok(())
}

fn generate(
    connection: &StorageConnection,
    store_id: &str,
    user_id: &str,
    InsertRequestRequisition {
        id,
        other_party_id,
        colour,
        comment,
        their_reference,
        max_months_of_stock,
        min_months_of_stock,
    }: InsertRequestRequisition,
) -> Result<RequisitionRow, RepositoryError> {
    let result = RequisitionRow {
        id,
        user_id: Some(user_id.to_string()),
        requisition_number: next_number(connection, &NumberRowType::RequestRequisition, &store_id)?,
        name_id: other_party_id,
        store_id: store_id.to_owned(),
        r#type: RequisitionRowType::Request,
        status: RequisitionRowStatus::Draft,
        created_datetime: Utc::now().naive_utc(),
        sent_datetime: None,
        finalised_datetime: None,
        colour,
        comment,
        their_reference,
        max_months_of_stock,
        min_months_of_stock,
        linked_requisition_id: None,
    };

    Ok(result)
}

impl From<RepositoryError> for InsertRequestRequisitionError {
    fn from(error: RepositoryError) -> Self {
        InsertRequestRequisitionError::DatabaseError(error)
    }
}

#[cfg(test)]
mod test_insert {
    use crate::{
        requisition::request_requisition::{
            InsertRequestRequisition, InsertRequestRequisitionError as ServiceError,
        },
        service_provider::ServiceProvider,
    };
    use chrono::Utc;
    use repository::{
        mock::{
            mock_name_a, mock_name_store_b, mock_name_store_c, mock_request_draft_requisition,
            mock_store_a, mock_user_account_a, MockData, MockDataInserts,
        },
        schema::{NameRow, RequisitionRow, RequisitionRowStatus, RequisitionRowType},
        test_db::{setup_all, setup_all_with_data},
        RequisitionRowRepository,
    };
    use util::inline_init;

    #[actix_rt::test]
    async fn insert_request_requisition_errors() {
        fn not_visible() -> NameRow {
            inline_init(|r: &mut NameRow| {
                r.id = "not_visible".to_string();
            })
        }

        let (_, _, connection_manager, _) = setup_all_with_data(
            "insert_request_requisition_errors",
            MockDataInserts::all(),
            Some(inline_init(|r: &mut MockData| {
                r.names = vec![not_visible()];
            })),
        )
        .await;

        let service_provider = ServiceProvider::new(connection_manager);
        let context = service_provider.context().unwrap();
        let service = service_provider.requisition_service;

        // RequisitionAlreadyExists
        assert_eq!(
            service.insert_request_requisition(
                &context,
                "store_a",
                "n/a",
                InsertRequestRequisition {
                    id: mock_request_draft_requisition().id,
                    other_party_id: "n/a".to_owned(),
                    colour: None,
                    their_reference: None,
                    comment: None,
                    max_months_of_stock: 1.0,
                    min_months_of_stock: 0.5,
                },
            ),
            Err(ServiceError::RequisitionAlreadyExists)
        );

        let name_store_b = mock_name_store_b();
        // OtherPartyNotASupplier
        assert_eq!(
            service.insert_request_requisition(
                &context,
                "store_a",
                "n/a",
                InsertRequestRequisition {
                    id: "new_request_requisition".to_owned(),
                    other_party_id: name_store_b.id.clone(),
                    colour: None,
                    their_reference: None,
                    comment: None,
                    max_months_of_stock: 1.0,
                    min_months_of_stock: 0.5,
                },
            ),
            Err(ServiceError::OtherPartyNotASupplier)
        );

        // OtherPartyNotVisible
        assert_eq!(
            service.insert_request_requisition(
                &context,
                &mock_store_a().id,
                "n/a",
                inline_init(|r: &mut InsertRequestRequisition| {
                    r.id = "new_id".to_string();
                    r.other_party_id = not_visible().id;
                })
            ),
            Err(ServiceError::OtherPartyNotVisible)
        );

        // OtherPartyDoesNotExist
        assert_eq!(
            service.insert_request_requisition(
                &context,
                "store_a",
                "n/a",
                InsertRequestRequisition {
                    id: "new_request_requisition".to_owned(),
                    other_party_id: "invalid".to_owned(),
                    colour: None,
                    their_reference: None,
                    comment: None,
                    max_months_of_stock: 1.0,
                    min_months_of_stock: 0.5,
                },
            ),
            Err(ServiceError::OtherPartyDoesNotExist)
        );

        // OtherPartyIsNotAStore
        assert_eq!(
            service.insert_request_requisition(
                &context,
                "store_c",
                "n/a",
                InsertRequestRequisition {
                    id: "new_request_requisition".to_owned(),
                    other_party_id: mock_name_a().id,
                    colour: None,
                    their_reference: None,
                    comment: None,
                    max_months_of_stock: 1.0,
                    min_months_of_stock: 0.5,
                },
            ),
            Err(ServiceError::OtherPartyIsNotAStore)
        );
        // Cannot be an error, names are filtered so that name linked to current store is not shown
        // OtherPartyIsThisStore
    }

    #[actix_rt::test]
    async fn insert_request_requisition_success() {
        let (_, connection, connection_manager, _) =
            setup_all("insert_request_requisition_success", MockDataInserts::all()).await;

        let service_provider = ServiceProvider::new(connection_manager);
        let context = service_provider.context().unwrap();
        let service = service_provider.requisition_service;

        let before_insert = Utc::now().naive_utc();

        let result = service
            .insert_request_requisition(
                &context,
                "store_a",
                &mock_user_account_a().id,
                InsertRequestRequisition {
                    id: "new_request_requisition".to_owned(),
                    other_party_id: mock_name_store_c().id,
                    colour: Some("new colour".to_owned()),
                    their_reference: Some("new their_reference".to_owned()),
                    comment: Some("new comment".to_owned()),
                    max_months_of_stock: 1.0,
                    min_months_of_stock: 0.5,
                },
            )
            .unwrap();

        let after_insert = Utc::now().naive_utc();

        let RequisitionRow {
            id,
            user_id,
            requisition_number: _,
            name_id,
            store_id,
            r#type,
            status,
            created_datetime,
            sent_datetime,
            finalised_datetime,
            colour,
            comment,
            their_reference,
            max_months_of_stock,
            min_months_of_stock,
            linked_requisition_id,
        } = RequisitionRowRepository::new(&connection)
            .find_one_by_id(&result.requisition_row.id)
            .unwrap()
            .unwrap();

        assert_eq!(id, "new_request_requisition".to_owned());
        assert_eq!(user_id, Some(mock_user_account_a().id));
        assert_eq!(name_id, mock_name_store_c().id);
        assert_eq!(colour, Some("new colour".to_owned()));
        assert_eq!(their_reference, Some("new their_reference".to_owned()));
        assert_eq!(comment, Some("new comment".to_owned()));
        assert_eq!(max_months_of_stock, 1.0);
        assert_eq!(min_months_of_stock, 0.5);
        assert_eq!(store_id, "store_a".to_owned());
        assert_eq!(r#type, RequisitionRowType::Request);
        assert_eq!(status, RequisitionRowStatus::Draft);
        assert!(created_datetime > before_insert && created_datetime < after_insert);
        assert_eq!(sent_datetime, None);
        assert_eq!(finalised_datetime, None);
        assert_eq!(linked_requisition_id, None);
    }
}

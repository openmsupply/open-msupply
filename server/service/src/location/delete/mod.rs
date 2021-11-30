mod validate;

use domain::{invoice_line::InvoiceLine, location::DeleteLocation, stock_line::StockLine};

use repository::{LocationRowRepository, RepositoryError};
use validate::validate;

use crate::{service_provider::ServiceConnection, WithDBError};

pub trait DeleteLocationServiceTrait {
    fn delete_location(&self, input: DeleteLocation) -> Result<String, DeleteLocationError>;
}

pub struct DeleteLocationService<'a>(pub ServiceConnection<'a>);

impl<'a> DeleteLocationServiceTrait for DeleteLocationService<'a> {
    fn delete_location(&self, input: DeleteLocation) -> Result<String, DeleteLocationError> {
        let location_id = self.0.transaction(|connection| {
            validate(&input, &connection)?;
            match LocationRowRepository::new(&connection).delete(&input.id) {
                Ok(_) => Ok(input.id),
                Err(err) => Err(DeleteLocationError::from(err)),
            }
        })?;
        Ok(location_id)
    }
}

#[derive(PartialEq, Debug)]
pub enum DeleteLocationError {
    LocationDoesNotExist,
    LocationDoesNotBelongToCurrentStore,
    LocationInUse {
        stock_lines: Vec<StockLine>,
        invoice_lines: Vec<InvoiceLine>,
    },
    DatabaseError(RepositoryError),
}

impl From<RepositoryError> for DeleteLocationError {
    fn from(error: RepositoryError) -> Self {
        DeleteLocationError::DatabaseError(error)
    }
}

impl<E> From<WithDBError<E>> for DeleteLocationError
where
    E: Into<DeleteLocationError>,
{
    fn from(result: WithDBError<E>) -> Self {
        match result {
            WithDBError::DatabaseError(error) => error.into(),
            WithDBError::Error(error) => error.into(),
        }
    }
}

#[cfg(test)]
mod tests;

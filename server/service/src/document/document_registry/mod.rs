use repository::{
    DocumentRegistry, DocumentRegistryFilter, DocumentRegistryRepository, EqualFilter, Pagination,
    RepositoryError,
};

use crate::service_provider::ServiceContext;

pub use self::insert::*;

mod insert;

#[cfg(test)]
mod tests;

pub trait DocumentRegistryServiceTrait: Sync + Send {
    fn get_entries(&self, ctx: &ServiceContext) -> Result<Vec<DocumentRegistry>, RepositoryError> {
        let repo = DocumentRegistryRepository::new(&ctx.connection);
        Ok(repo.query(Pagination::new(), None, None)?)
    }

    fn get_children(
        &self,
        ctx: &ServiceContext,
        parent_ids: &[String],
    ) -> Result<Vec<DocumentRegistry>, RepositoryError> {
        let repo = DocumentRegistryRepository::new(&ctx.connection);
        Ok(repo.query(
            Pagination::new(),
            Some(
                DocumentRegistryFilter::new()
                    .parent_id(EqualFilter::equal_any(parent_ids.to_vec())),
            ),
            None,
        )?)
    }

    fn insert(
        &self,
        ctx: &ServiceContext,
        input: InsertDocumentRegistry,
    ) -> Result<DocumentRegistry, InsertDocRegistryError> {
        insert(ctx, input)
    }
}

pub struct DocumentRegistryService {}
impl DocumentRegistryServiceTrait for DocumentRegistryService {}
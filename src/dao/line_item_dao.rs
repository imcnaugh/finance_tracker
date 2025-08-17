use crate::dao::crud::Crud;
use crate::model::line_item::LineItem;
use sqlx::Executor;
use std::error::Error;

/// A trait for data access operations specific to line items in an invoice system.
/// Extends the base CRUD operations with line-item-specific functionality.
pub trait LineItemDao: Crud<LineItem> {
    /// Retrieves all line items associated with a specific invoice.
    ///
    /// # Arguments
    /// * `executor` - The database executor to use for the query
    /// * `invoice_id` - The ID of the invoice to get line items for
    ///
    /// # Returns
    /// A Result containing a vector of LineItem objects if successful,
    /// or an error if the operation fails
    async fn get_line_items_for_invoice<'e, E>(
        &self,
        executor: E,
        invoice_id: &str,
    ) -> Result<Vec<LineItem>, Box<dyn Error>>
    where
        E: Executor<'e, Database = Self::DB>;
}

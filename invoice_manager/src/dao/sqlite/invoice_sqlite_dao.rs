use crate::dao::invoice_dao::InvoiceDao;
use crate::dao::sqlite::sqlite_connection::get_pooled_connection;
use crate::model::invoice::Invoice;
use crate::model::invoice_status::InvoiceStatus;
use crate::model::line_item::LineItem;
use crate::model::{InvoiceSearch, NewInvoice, NewLineItem};
use sqlx::{Error, Executor, Sqlite};

pub struct InvoiceSqliteDao;

const INVOICE_INSERT_SQL: &str = r#"
INSERT INTO invoice (
    id,
    client_id,
    draft_date,
    due_date,
    sent_date,
    paid_date,
    cancelled_date
) VALUES (?, ?, ?, ?, ?, ?, ?)
"#;

const LINE_ITEM_INSERT_SQL: &str = r#"
INSERT INTO line_item (
    id,
    description,
    quantity,
    unit_price_in_cents,
    created_timestamp,
    invoice_id
    ) VALUES (?, ?, ?, ?, ?, ?)
"#;

const INVOICE_SELECT_BY_ID_SQL: &str = r#"
SELECT
    id,
    client_id,
    draft_date,
    due_date,
    sent_date,
    paid_date,
    cancelled_date
FROM invoice
WHERE id = ?
"#;

const INVOICE_UPDATE_SQL: &str = r#"
UPDATE invoice
SET
    client_id = ?,
    draft_date = ?,
    due_date = ?,
    sent_date = ?,
    paid_date = ?,
    cancelled_date = ?
WHERE id = ?
"#;

const INVOICE_DELETE_SQL: &str = r#"
DELETE FROM invoice
WHERE id = ?
"#;

const LINE_ITEM_SELECT_BY_INVOICE_ID_SQL: &str = r#"
SELECT
    id,
    description,
    quantity,
    unit_price_in_cents,
    created_timestamp
FROM line_item
WHERE invoice_id = ?
"#;

impl InvoiceSqliteDao {
    pub fn new() -> Self {
        Self {}
    }

    async fn insert_invoice<'e, E>(&self, executor: E, item: &Invoice) -> Result<(), Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let draft_timestamp = item
            .get_draft_date()
            .map_err(|_| Error::Decode("Invalid draft_date".into()))?;
        let sent_timestamp = Self::map_to_slqx_error(item.get_sent_date(), "sent date")?;
        let paid_timestamp = Self::map_to_slqx_error(item.get_paid_date(), "paid date")?;
        let due_timestamp = Self::map_to_slqx_error(item.get_due_date(), "due date")?;
        let cancelled_timestamp =
            Self::map_to_slqx_error(item.get_cancelled_date(), "cancelled date")?;

        let query = sqlx::query(INVOICE_INSERT_SQL)
            .bind(item.get_id())
            .bind(item.get_client_id())
            .bind(draft_timestamp.timestamp())
            .bind(due_timestamp.map(|d| d.timestamp()))
            .bind(sent_timestamp.map(|d| d.timestamp()))
            .bind(paid_timestamp.map(|d| d.timestamp()))
            .bind(cancelled_timestamp.map(|d| d.timestamp()));

        query.execute(executor).await?;
        Ok(())
    }

    async fn insert_line_item<'e, E>(&self, executor: E, item: &LineItem) -> Result<(), Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let query = sqlx::query(LINE_ITEM_INSERT_SQL)
            .bind(item.get_id())
            .bind(item.get_description())
            .bind(item.get_quantity())
            .bind(item.get_unit_price_in_cents())
            .bind(item.get_invoice_id());

        query.execute(executor).await?;
        Ok(())
    }

    async fn select_invoice_by_id<'e, E>(
        &self,
        executor: E,
        id: &str,
    ) -> Result<Option<Invoice>, Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let item = sqlx::query_as::<_, Invoice>(INVOICE_SELECT_BY_ID_SQL)
            .bind(id)
            .fetch_optional(executor)
            .await?;

        Ok(item)
    }

    async fn update_invoice<'e, E>(
        &self,
        executor: E,
        id: &str,
        item: &Invoice,
    ) -> Result<(), Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let draft_timestamp = item
            .get_draft_date()
            .map_err(|_| Error::Decode("Invalid draft_date".into()))?;
        let sent_timestamp = Self::map_to_slqx_error(item.get_sent_date(), "sent date")?;
        let paid_timestamp = Self::map_to_slqx_error(item.get_paid_date(), "paid date")?;
        let due_timestamp = Self::map_to_slqx_error(item.get_due_date(), "due date")?;
        let cancelled_timestamp =
            Self::map_to_slqx_error(item.get_cancelled_date(), "cancelled date")?;

        let query = sqlx::query(INVOICE_UPDATE_SQL)
            .bind(item.get_client_id())
            .bind(draft_timestamp.timestamp())
            .bind(due_timestamp.map(|d| d.timestamp()))
            .bind(sent_timestamp.map(|d| d.timestamp()))
            .bind(paid_timestamp.map(|d| d.timestamp()))
            .bind(cancelled_timestamp.map(|d| d.timestamp()))
            .bind(id);

        query.execute(executor).await?;
        Ok(())
    }

    async fn delete_invoice<'e, E>(&self, executor: E, id: &str) -> Result<(), Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let query = sqlx::query(INVOICE_DELETE_SQL).bind(id);

        query.execute(executor).await?;
        Ok(())
    }

    async fn read_line_items_by_invoice_id<'e, E>(
        &self,
        executor: E,
        invoice_id: &str,
    ) -> Result<Vec<LineItem>, Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let query =
            sqlx::query_as::<_, LineItem>(LINE_ITEM_SELECT_BY_INVOICE_ID_SQL).bind(invoice_id);
        Ok(query.fetch_all(executor).await?)
    }

    fn map_to_slqx_error<T>(r: Result<Option<T>, ()>, field: &str) -> Result<Option<T>, Error> {
        r.map_err(|_| Error::Decode(format!("Invalid {} value", field).into()))
    }

    async fn search_invoices<'e, E>(
        &self,
        executor: E,
        search_terms: &InvoiceSearch,
    ) -> Result<Vec<Invoice>, Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let mut statement = String::from("SELECT * FROM invoice WHERE 1 = 1");

        if let Some(_) = search_terms.get_client_id() {
            statement.push_str(" AND client_id = ?");
        }

        if let Some(status) = search_terms.get_status() {
            match status {
                InvoiceStatus::DRAFT => {
                    statement.push_str(" AND sent_date IS NULL AND cancelled_date IS NULL")
                }
                InvoiceStatus::SENT => {
                    statement.push_str(" AND sent_date IS NOT NULL AND cancelled_date IS NOT NULL AND paid_date IS NULL")
                }
                InvoiceStatus::PAID => {
                    statement.push_str(" AND paid_date IS NOT NULL AND cancelled_date IS NOT NULL")
                }
                InvoiceStatus::OVERDUE => {
                    statement.push_str(" AND paid_date IS NULL AND cancelled_date IS NULL AND due_date < unixepoch()")
                }
                InvoiceStatus::CANCELLED => {
                    statement.push_str(" AND cancelled_date IS NOT NULL")
                }
            }
        }

        if let Some(_) = search_terms.get_draft_date_range() {
            statement.push_str(" AND draft_date BETWEEN ? AND ?")
        }
        if let Some(_) = search_terms.get_sent_date_range() {
            statement.push_str(" AND sent_date BETWEEN ? AND ?")
        }
        if let Some(_) = search_terms.get_paid_date_range() {
            statement.push_str(" AND paid_date BETWEEN ? AND ?")
        }
        if let Some(_) = search_terms.get_due_date_range() {
            statement.push_str(" AND due_date BETWEEN ? AND ?")
        }
        if let Some(_) = search_terms.get_canceled_date_range() {
            statement.push_str(" AND cancelled_date BETWEEN ? AND ?")
        }

        let mut query = sqlx::query_as::<_, Invoice>(&statement);

        if let Some(client_id) = search_terms.get_client_id() {
            query = query.bind(client_id);
        }
        if let Some(draft_date_range) = search_terms.get_draft_date_range() {
            query = query
                .bind(draft_date_range.get_start_date().timestamp())
                .bind(draft_date_range.get_end_date().timestamp());
        }
        if let Some(sent_date_range) = search_terms.get_sent_date_range() {
            query = query
                .bind(sent_date_range.get_start_date().timestamp())
                .bind(sent_date_range.get_end_date().timestamp());
        }
        if let Some(paid_date_range) = search_terms.get_paid_date_range() {
            query = query
                .bind(paid_date_range.get_start_date().timestamp())
                .bind(paid_date_range.get_end_date().timestamp());
        }
        if let Some(due_date_range) = search_terms.get_due_date_range() {
            query = query
                .bind(due_date_range.get_start_date().timestamp())
                .bind(due_date_range.get_end_date().timestamp());
        }
        if let Some(canceled_date_range) = search_terms.get_canceled_date_range() {
            query = query
                .bind(canceled_date_range.get_start_date().timestamp())
                .bind(canceled_date_range.get_end_date().timestamp());
        }

        Ok(query.fetch_all(executor).await?)
    }
}

impl InvoiceDao for InvoiceSqliteDao {
    async fn create_invoice(&self, new_invoice: &NewInvoice) -> Result<Invoice, Error> {
        let mut conn = get_pooled_connection().await?;

        let new_invoice = Invoice::from(new_invoice);
        self.insert_invoice(&mut *conn, &new_invoice).await?;

        Ok(new_invoice)
    }

    async fn get_invoice(&self, id: &str) -> Result<Option<Invoice>, Error> {
        let mut conn = get_pooled_connection().await?;
        let invoice_result = self.select_invoice_by_id(&mut *conn, id).await?;

        match invoice_result {
            None => Ok(None),
            Some(mut invoice) => {
                let line_items = self.read_line_items_by_invoice_id(&mut *conn, id).await?;
                invoice.set_line_items(line_items);
                Ok(Some(invoice))
            }
        }
    }

    async fn search_invoices(&self, search_terms: &InvoiceSearch) -> Result<Vec<Invoice>, Error> {
        let mut conn = get_pooled_connection().await?;
        match self.search_invoices(&mut *conn, search_terms).await {
            Ok(mut invoices) => {
                // TODO make a sql call to get line items for many invoices at once
                for invoice in invoices.iter_mut() {
                    let line_items = self
                        .read_line_items_by_invoice_id(&mut *conn, &invoice.get_id())
                        .await?;
                    invoice.set_line_items(line_items);
                }
                Ok(invoices)
            }
            Err(e) => Err(e),
        }
    }

    async fn create_line_item(
        &self,
        invoice_id: &str,
        new_line_item: &NewLineItem,
    ) -> Result<LineItem, Error> {
        let mut conn = get_pooled_connection().await?;
        let new_line_item = LineItem::from((new_line_item, invoice_id));
        self.insert_line_item(&mut *conn, &new_line_item).await?;
        Ok(new_line_item)
    }
}

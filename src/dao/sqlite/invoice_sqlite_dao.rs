use crate::dao::invoice_dao::InvoiceDao;
use crate::dao::sqlite::sqlite_connection::get_pooled_connection;
use crate::model::NewInvoice;
use crate::model::invoice::Invoice;
use crate::model::line_item::LineItem;
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

    async fn create<'e, E>(&self, executor: E, item: &Invoice) -> Result<(), Error>
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

    async fn read<'e, E>(&self, executor: E, id: &str) -> Result<Option<Invoice>, Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let item = sqlx::query_as::<_, Invoice>(INVOICE_SELECT_BY_ID_SQL)
            .bind(id)
            .fetch_optional(executor)
            .await?;

        Ok(item)
    }

    async fn update<'e, E>(&self, executor: E, id: &str, item: &Invoice) -> Result<(), Error>
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

    async fn delete<'e, E>(&self, executor: E, id: &str) -> Result<(), Error>
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
    ) -> Result<Vec<LineItem>, sqlx::Error>
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
}

impl InvoiceDao for InvoiceSqliteDao {
    async fn create_invoice(&self, new_invoice: &NewInvoice) -> Result<Invoice, Error> {
        let mut conn = get_pooled_connection().await?;

        let new_invoice = Invoice::from(new_invoice);
        self.create(&mut *conn, &new_invoice).await?;

        Ok(new_invoice)
    }

    async fn get_invoice(&self, id: &str) -> Result<Option<Invoice>, Error> {
        let mut conn = get_pooled_connection().await?;
        let invoice_result = self.read(&mut *conn, id).await?;

        match invoice_result {
            None => Ok(None),
            Some(mut invoice) => {
                let line_items = self.read_line_items_by_invoice_id(&mut *conn, id).await?;
                invoice.set_line_items(line_items);
                Ok(Some(invoice))
            }
        }
    }

    async fn search_invoices(&self) -> Result<Vec<Invoice>, Error> {
        todo!()
    }
}

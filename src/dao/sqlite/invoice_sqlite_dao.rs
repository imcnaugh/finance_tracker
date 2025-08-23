use crate::dao::invoice_dao::InvoiceDao;
use crate::dao::sqlite::sqlite_connection::get_pooled_connection;
use crate::model::NewInvoice;
use crate::model::invoice::Invoice;
use sqlx::{Acquire, Executor, Sqlite};

pub struct InvoiceSqliteDao;

const INVOICE_INSERT_SQL: &str = r#"
INSERT INTO invoice (
    id,
    client_id,
    draft_date,
    sent_date,
    paid_date,
    cancelled_date
) VALUES (?, ?, ?, ?, ?, ?)
"#;

const INVOICE_SELECT_BY_ID_SQL: &str = r#"
SELECT
    id,
    client_id,
    draft_date,
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

    async fn create<'e, E>(&self, executor: E, item: &Invoice) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let query = sqlx::query(INVOICE_INSERT_SQL)
            .bind(item.get_id())
            .bind(item.get_client_id())
            .bind(item.get_draft_date().timestamp())
            .bind(item.get_sent_date().map(|d| d.timestamp()))
            .bind(item.get_paid_date().map(|d| d.timestamp()))
            .bind(item.get_cancelled_date().map(|d| d.timestamp()));

        query.execute(executor).await?;
        Ok(())
    }

    async fn read<'e, E>(&self, executor: E, id: &str) -> Result<Option<Invoice>, sqlx::Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let item = sqlx::query_as::<_, Invoice>(INVOICE_SELECT_BY_ID_SQL)
            .bind(id)
            .fetch_optional(executor)
            .await?;

        Ok(item)
    }

    async fn update<'e, E>(&self, executor: E, id: &str, item: &Invoice) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let query = sqlx::query(INVOICE_UPDATE_SQL)
            .bind(item.get_client_id())
            .bind(item.get_draft_date().timestamp())
            .bind(item.get_sent_date().map(|d| d.timestamp()))
            .bind(item.get_paid_date().map(|d| d.timestamp()))
            .bind(item.get_cancelled_date().map(|d| d.timestamp()))
            .bind(id);

        query.execute(executor).await?;
        Ok(())
    }

    async fn delete<'e, E>(&self, executor: E, id: &str) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let query = sqlx::query(INVOICE_DELETE_SQL).bind(id);

        query.execute(executor).await?;
        Ok(())
    }
}

impl InvoiceDao for InvoiceSqliteDao {
    async fn create_invoice(&self, new_invoice: &NewInvoice) -> Result<Invoice, sqlx::Error> {
        let mut conn = get_pooled_connection().await?;
        let tx = conn.acquire().await?;

        let new_invoice = Invoice::from(new_invoice);
        self.create(tx, &new_invoice).await?;

        Ok(new_invoice)
    }

    async fn get_invoice(&self, id: &str) -> Result<Option<Invoice>, sqlx::Error> {
        let mut conn = get_pooled_connection().await?;
        let tx = conn.acquire().await?;
        Ok(self.read(tx, id).await?)
    }

    async fn search_invoices(&self) -> Result<Vec<Invoice>, sqlx::Error> {
        todo!()
    }
}

use invoice_manager::dao::invoice_dao::InvoiceDao;
use invoice_manager::model::Invoice;
use invoice_manager::model::InvoiceStatus;
use invoice_manager::model::LineItem;
use invoice_manager::model::{InvoiceSearch, NewInvoice, NewLineItem};
use sqlx::{Error, Executor, Pool, Sqlite};

pub struct InvoiceSqliteDao {
    pool: Pool<Sqlite>,
}

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

const INVOICE_SET_SENT_DATE_SQL: &str = r#"
UPDATE invoice
SET sent_date = ?
WHERE id = ?
"#;

const INVOICE_SET_PAID_DATE_SQL: &str = r#"
UPDATE invoice
SET paid_date = ?
WHERE id = ?
"#;

const INVOICE_SET_CANCELLED_DATE_SQL: &str = r#"
UPDATE invoice
SET cancelled_date = ?
WHERE id = ?
"#;

const LINE_ITEM_INSERT_SQL: &str = r#"
INSERT INTO line_item (
    id,
    description,
    quantity,
    unit_price_in_cents,
    invoice_id
    ) VALUES (?, ?, ?, ?, ?)
"#;

const LINE_ITEM_DELETE_SQL: &str = r#"
DELETE FROM line_item
WHERE id = ?
AND invoice_id = ?
"#;

const LINE_ITEM_SELECT_BY_INVOICE_ID_SQL: &str = r#"
SELECT
    id,
    description,
    quantity,
    unit_price_in_cents,
    created_timestamp,
    invoice_id
FROM line_item
WHERE invoice_id = ?
"#;

impl InvoiceSqliteDao {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
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

    async fn invoice_set_status_date<'e, E>(
        &self,
        executor: E,
        id: &str,
        date_type: InvoiceStatus,
        date: i64,
    ) -> Result<(), Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let sql = match date_type {
            InvoiceStatus::SENT => INVOICE_SET_SENT_DATE_SQL,
            InvoiceStatus::PAID => INVOICE_SET_PAID_DATE_SQL,
            InvoiceStatus::CANCELLED => INVOICE_SET_CANCELLED_DATE_SQL,
            t => {
                return Err(Error::Decode(
                    format!("Unable to set date for status type {:?}", t).into(),
                ));
            }
        };
        let query = sqlx::query(sql).bind(date).bind(id);
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
        query.fetch_all(executor).await
    }

    fn map_to_slqx_error<T>(
        r: Result<Option<T>, utilities::Error>,
        field: &str,
    ) -> Result<Option<T>, Error> {
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

        if search_terms.get_client_id().is_some() {
            statement.push_str(" AND client_id = ?");
        }

        if let Some(status) = search_terms.get_status() {
            match status {
                InvoiceStatus::DRAFT => {
                    statement.push_str(" AND sent_date IS NULL AND cancelled_date IS NULL")
                }
                InvoiceStatus::SENT => statement.push_str(
                    " AND sent_date IS NOT NULL AND cancelled_date IS NULL AND paid_date IS NULL",
                ),
                InvoiceStatus::PAID => {
                    statement.push_str(" AND paid_date IS NOT NULL AND cancelled_date IS NULL")
                }
                InvoiceStatus::OVERDUE => statement.push_str(
                    " AND paid_date IS NULL AND cancelled_date IS NULL AND due_date < unixepoch()",
                ),
                InvoiceStatus::CANCELLED => statement.push_str(" AND cancelled_date IS NOT NULL"),
            }
        }

        if search_terms.get_draft_date_range().is_some() {
            statement.push_str(" AND draft_date BETWEEN ? AND ?")
        }
        if search_terms.get_sent_date_range().is_some() {
            statement.push_str(" AND sent_date BETWEEN ? AND ?")
        }
        if search_terms.get_paid_date_range().is_some() {
            statement.push_str(" AND paid_date BETWEEN ? AND ?")
        }
        if search_terms.get_due_date_range().is_some() {
            statement.push_str(" AND due_date BETWEEN ? AND ?")
        }
        if search_terms.get_canceled_date_range().is_some() {
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

        query.fetch_all(executor).await
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

    async fn delete_line_item<'e, E>(
        &self,
        executor: E,
        invoice_id: &str,
        line_item_id: &str,
    ) -> Result<(), Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let query = sqlx::query(LINE_ITEM_DELETE_SQL)
            .bind(line_item_id)
            .bind(invoice_id);

        query.execute(executor).await?;
        Ok(())
    }
}

impl InvoiceDao for InvoiceSqliteDao {
    async fn create_invoice(&self, new_invoice: &NewInvoice) -> Result<Invoice, Error> {
        let mut conn = self.pool.acquire().await?;

        let new_invoice = Invoice::from(new_invoice);
        self.insert_invoice(&mut *conn, &new_invoice).await?;

        Ok(new_invoice)
    }

    async fn get_invoice(&self, id: &str) -> Result<Option<Invoice>, Error> {
        let mut conn = self.pool.acquire().await?;
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

    async fn set_invoice_status_timestamp(
        &self,
        id: &str,
        sent_date: i64,
        status: InvoiceStatus,
    ) -> Result<Invoice, Error> {
        let mut conn = self.pool.acquire().await?;
        self.invoice_set_status_date(&mut *conn, id, status, sent_date)
            .await?;
        let mut invoice = self.select_invoice_by_id(&mut *conn, id).await?.unwrap();
        let line_items = self.read_line_items_by_invoice_id(&mut *conn, id).await?;
        invoice.set_line_items(line_items);
        Ok(invoice)
    }

    async fn search_invoices(&self, search_terms: &InvoiceSearch) -> Result<Vec<Invoice>, Error> {
        let mut conn = self.pool.acquire().await?;
        match self.search_invoices(&mut *conn, search_terms).await {
            Ok(mut invoices) => {
                // TODO make a sql call to get line items for many invoices at once
                for invoice in invoices.iter_mut() {
                    let line_items = self
                        .read_line_items_by_invoice_id(&mut *conn, invoice.get_id())
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
    ) -> Result<Invoice, Error> {
        let mut conn = self.pool.acquire().await?;
        let new_line_item = LineItem::from((new_line_item, invoice_id));
        self.insert_line_item(&mut *conn, &new_line_item).await?;
        let invoice = self.get_invoice(invoice_id).await?.unwrap();
        Ok(invoice)
    }

    async fn delete_line_item(
        &self,
        invoice_id: &str,
        line_item_id: &str,
    ) -> Result<Invoice, Error> {
        let mut conn = self.pool.acquire().await?;
        self.delete_line_item(&mut *conn, invoice_id, line_item_id)
            .await?;
        let invoice = self.get_invoice(invoice_id).await?.unwrap();
        Ok(invoice)
    }
}

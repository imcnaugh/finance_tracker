use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "UPPERCASE")]
pub enum InvoiceStatus {
    DRAFT,
    SENT,
    PAID,
    OVERDUE,
    CANCELLED,
}

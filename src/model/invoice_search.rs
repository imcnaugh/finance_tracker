use crate::model::invoice_status::InvoiceStatus;
use chrono::{DateTime, TimeDelta, Utc};

pub struct InvoiceSearch {
    client_id: Option<String>,
    status: Option<InvoiceStatus>,
    sent_date_range: Option<DateRange>,
}

pub struct DateRange {
    date: DateTime<Utc>,
    duration: TimeDelta,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
}

impl DateRange {
    pub fn new(date: DateTime<Utc>, duration: TimeDelta) -> Self {
        let (start_date, end_date) = if duration.num_seconds() > 0 {
            (date, date + duration)
        } else {
            (date - duration, date)
        };

        Self {
            date,
            duration,
            start_date,
            end_date,
        }
    }

    pub fn get_start_date(&self) -> DateTime<Utc> {
        self.start_date
    }

    pub fn get_end_date(&self) -> DateTime<Utc> {
        self.end_date
    }

    pub fn get_duration(&self) -> TimeDelta {
        self.duration
    }
}

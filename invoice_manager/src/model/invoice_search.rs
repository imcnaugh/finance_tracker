use crate::model::invoice_status::InvoiceStatus;
use chrono::{DateTime, Duration, NaiveDate, TimeDelta, Utc};
use clap::Args;
use std::str::FromStr;

#[derive(Args, Debug, Clone, Default)]
pub struct InvoiceSearch {
    /// Client ID
    #[arg(long)]
    client_id: Option<String>,

    /// Invoice status
    #[arg(long)]
    status: Option<InvoiceStatus>,

    /// Date range for invoices drafted between dates
    #[arg(long, value_name = "YYYY-MM-DD..YYYY-MM-DD")]
    draft_date_range: Option<DateRange>,

    /// Date range for invoices sent between dates
    #[arg(long, value_name = "YYYY-MM-DD..YYYY-MM-DD")]
    sent_date_range: Option<DateRange>,

    /// Date range for invoices paid between dates
    #[arg(long, value_name = "YYYY-MM-DD..YYYY-MM-DD")]
    paid_date_range: Option<DateRange>,

    /// Date range for invoices due between dates
    #[arg(long, value_name = "YYYY-MM-DD..YYYY-MM-DD")]
    due_date_range: Option<DateRange>,

    /// Date range for invoices canceled between dates
    #[arg(long, value_name = "YYYY-MM-DD..YYYY-MM-DD")]
    canceled_date_range: Option<DateRange>,
}

#[derive(Debug, Clone)]
pub struct DateRange {
    duration: TimeDelta,
    start_date: DateTime<Utc>,
}

impl InvoiceSearch {
    pub fn new() -> Self {
        Self {
            client_id: None,
            status: None,
            draft_date_range: None,
            sent_date_range: None,
            paid_date_range: None,
            due_date_range: None,
            canceled_date_range: None,
        }
    }

    pub fn get_client_id(&self) -> &Option<String> {
        &self.client_id
    }

    pub fn set_client_id(&mut self, client_id: &str) {
        self.client_id = Some(client_id.into());
    }

    pub fn get_status(&self) -> &Option<InvoiceStatus> {
        &self.status
    }

    pub fn set_status(&mut self, status: &InvoiceStatus) {
        self.status = Some(*status);
    }

    pub fn get_draft_date_range(&self) -> &Option<DateRange> {
        &self.draft_date_range
    }

    pub fn set_draft_date_range(&mut self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) {
        self.draft_date_range = Some(DateRange::build(start_date, end_date));
    }

    pub fn get_sent_date_range(&self) -> &Option<DateRange> {
        &self.sent_date_range
    }

    pub fn set_sent_date_range(&mut self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) {
        self.sent_date_range = Some(DateRange::build(start_date, end_date));
    }

    pub fn get_paid_date_range(&self) -> &Option<DateRange> {
        &self.paid_date_range
    }

    pub fn set_paid_date_range(&mut self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) {
        self.paid_date_range = Some(DateRange::build(start_date, end_date));
    }

    pub fn get_due_date_range(&self) -> &Option<DateRange> {
        &self.due_date_range
    }

    pub fn set_due_date_range(&mut self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) {
        self.due_date_range = Some(DateRange::build(start_date, end_date));
    }

    pub fn get_canceled_date_range(&self) -> &Option<DateRange> {
        &self.canceled_date_range
    }

    pub fn set_canceled_date_range(&mut self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) {
        self.canceled_date_range = Some(DateRange::build(start_date, end_date));
    }
}

impl DateRange {
    pub fn new(date: DateTime<Utc>, duration: TimeDelta) -> Self {
        let start_date = if duration > Duration::zero() {
            date
        } else {
            date + duration
        };

        Self {
            duration: duration.abs(),
            start_date,
        }
    }

    pub fn build(d1: DateTime<Utc>, d2: DateTime<Utc>) -> Self {
        let duration = d2 - d1;
        if d1 > d2 {
            Self {
                duration: duration.abs(),
                start_date: d2,
            }
        } else {
            Self {
                duration: duration.abs(),
                start_date: d1,
            }
        }
    }

    pub fn get_start_date(&self) -> DateTime<Utc> {
        self.start_date
    }

    pub fn get_end_date(&self) -> DateTime<Utc> {
        self.start_date + self.duration
    }

    pub fn get_duration(&self) -> TimeDelta {
        self.duration.abs()
    }
}

impl FromStr for DateRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("..").collect();
        if parts.len() != 2 {
            return Err("Invalid date range format. Expected 'YYYY-MM-DD..YYYY-MM-DD'".into());
        }

        let start = NaiveDate::parse_from_str(parts[0], "%Y-%m-%d")
            .map_err(|e| format!("Invalid start date: {}", e))?;
        let end = NaiveDate::parse_from_str(parts[1], "%Y-%m-%d")
            .map_err(|e| format!("Invalid end date: {}", e))?;

        let start_date = start.and_hms_opt(0, 0, 0).unwrap().and_utc();
        let end_date = end.and_hms_opt(0, 0, 0).unwrap().and_utc();

        Ok(Self::build(start_date, end_date))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_range() {
        let date = Utc::now();
        let duration = TimeDelta::days(1);
        let date_range = DateRange::new(date, duration);

        assert_eq!(date_range.get_start_date(), date);
        assert_eq!(date_range.get_end_date(), date + duration);
        assert_eq!(date_range.get_duration(), duration);
    }

    #[test]
    fn test_date_range_negative_duration() {
        let date = Utc::now();
        let duration = TimeDelta::days(-1);
        let date_range = DateRange::new(date, duration);

        assert_eq!(date_range.get_start_date(), date + duration);
        assert_eq!(date_range.get_end_date(), date);
        assert_eq!(date_range.get_duration(), duration.abs());
    }

    #[test]
    fn test_date_range_build() {
        let date1 = Utc::now();
        let date2 = date1 + TimeDelta::days(1);
        let date_range = DateRange::build(date1, date2);

        assert_eq!(date_range.get_start_date(), date1);
        assert_eq!(date_range.get_end_date(), date2);
        assert_eq!(date_range.get_duration(), date2 - date1);
    }

    #[test]
    fn test_date_range_build_negative_duration() {
        let date1 = Utc::now();
        let date2 = date1 + TimeDelta::days(-1);
        let date_range = DateRange::build(date1, date2);

        assert_eq!(date_range.get_start_date(), date2);
        assert_eq!(date_range.get_end_date(), date1);
    }
}

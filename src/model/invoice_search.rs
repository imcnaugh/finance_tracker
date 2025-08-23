use crate::model::invoice_status::InvoiceStatus;
use chrono::{DateTime, Duration, TimeDelta, Utc};

pub struct InvoiceSearch {
    client_id: Option<String>,
    status: Option<InvoiceStatus>,
    draft_date_range: Option<DateRange>,
    sent_date_range: Option<DateRange>,
    paid_date_range: Option<DateRange>,
    overdue_date_range: Option<DateRange>,
    due_date_range: Option<DateRange>,
}

pub struct DateRange {
    duration: TimeDelta,
    start_date: DateTime<Utc>,
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

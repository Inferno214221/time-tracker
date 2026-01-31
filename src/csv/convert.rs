use serde::Serialize;

use crate::orm::query::TimeWithTickets;

#[derive(Debug, Serialize)]
pub struct CsvTime {
    pub time_start: String,
    pub time_end: String,
    pub time_dur: f64,
    pub tickets: String,
    pub time_desc: String,
}

impl From<TimeWithTickets> for CsvTime {
    fn from(value: TimeWithTickets) -> Self {
        CsvTime {
            time_start: value.time_start.to_string(),
            time_end: value.time_end.to_string(),
            time_dur: value.time_dur.unwrap(),
            time_desc: value.time_desc,
            tickets: value.tickets.into_iter()
                .map(|t| t.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        }
    }
}
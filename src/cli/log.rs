use std::str::FromStr;

use chrono::Local;
use diesel::prelude::*;

use crate::{cli::args::LogArgs, orm::{insert::LoggedTime, model::Ticket}};

pub fn log(conn: &mut SqliteConnection, args: LogArgs) {
    use crate::orm::schema::time;

    let date = args.date.unwrap_or_else(|| Local::now().date_naive());

    let log = LoggedTime {
        time_start: date.and_time(args.time_range.start),
        time_end: date.and_time(args.time_range.end),
        time_desc: args.description,
    };

    let id: i32 = log.insert_into(time::table)
        .returning(time::time_id)
        .get_result(conn)
        .unwrap();

    // TODO: Insert tickets as required.

    let tickets: Vec<Ticket> = args.tickets.iter()
        .map(|s| Ticket::from_str(s).unwrap_or_else(|e| panic!("{e}")))
        .collect();
}
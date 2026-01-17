use std::{iter, str::FromStr};

use chrono::Local;
use diesel::{insert_into, prelude::*};

use crate::{cli::args::LogArgs, orm::{insert::LoggedTime, model::{Ticket, TicketTime}}};

pub fn log(conn: &mut SqliteConnection, args: LogArgs) {
    use crate::orm::schema::{ticket_time, time};

    let date = args.date.unwrap_or_else(|| Local::now().date_naive());

    let log = LoggedTime {
        time_start: date.and_time(args.time_range.start),
        time_end: date.and_time(args.time_range.end),
        time_desc: args.description,
    };

    let id: i32 = log.insert_into(time::table)
        .returning(time::time_id)
        .get_result(conn)
        .expect("Error inserting time into database");

    // TODO: Scratch that, remove the ticket table entirely.

    let tickets: Vec<TicketTime> = args.tickets.iter()
        .map(|s| Ticket::from_str(s).unwrap_or_else(|e| panic!("{e}")))
        .zip(iter::repeat(id))
        .map(TicketTime::from)
        .collect();

    insert_into(ticket_time::table)
        .values(tickets)
        .execute(conn)
        .expect("Error inserting time into database");
}
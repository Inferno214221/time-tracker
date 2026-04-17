use std::iter;

use diesel::{insert_into, prelude::*};

use crate::{cli::args::LogArgs, orm::{insert::LoggedTime, model::TicketTime}, util::error::DynResult};

pub fn log(conn: &mut SqliteConnection, args: LogArgs) -> DynResult<()> {
    use crate::orm::schema::{ticket_time, time};

    let date = args.date.unwrap_or_default();

    let log = LoggedTime {
        time_start: date.and_time(args.time_range.start),
        time_end: date.and_time(args.time_range.end),
        time_desc: args.description,
        act_num: args.activity,
    };

    let id: i32 = log.insert_into(time::table)
        .returning(time::time_id)
        .get_result(conn)
        .map_err(|e| format!("Error inserting time into database:\n{e}"))?;

    let tickets: Vec<TicketTime> = args.tickets.into_iter()
        .zip(iter::repeat(id))
        .map(TicketTime::from)
        .collect();

    insert_into(ticket_time::table)
        .values(tickets)
        .execute(conn)
        .map_err(|e| format!("Error inserting ticket-time relations into database:\n{e}"))?;

    println!("Time logged successfully");

    Ok(())
}
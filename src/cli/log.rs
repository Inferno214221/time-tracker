use std::{error::Error, iter, str::FromStr};

use chrono::Local;
use diesel::{insert_into, prelude::*};

use crate::{cli::args::LogArgs, orm::{insert::LoggedTime, model::{InvoiceActivity, TicketTime}, ticket::Ticket}};

pub fn log(conn: &mut SqliteConnection, args: LogArgs) -> Result<(), Box<dyn Error>> {
    use crate::orm::schema::{invoice_activity, ticket_time, time};

    let date = args.date.unwrap_or_else(|| Local::now().date_naive());

    let act_num = args.activity.or_else(
        || InvoiceActivity::query()
            .order_by(invoice_activity::act_num.desc())
            .get_result(conn)
            .map(|a| a.act_num)
            .ok()
    ).ok_or("Error retrieving most recent activity")?;

    let log = LoggedTime {
        time_start: date.and_time(args.time_range.start),
        time_end: date.and_time(args.time_range.end),
        time_desc: args.description,
        act_num,
    };

    let id: i32 = log.insert_into(time::table)
        .returning(time::time_id)
        .get_result(conn)
        .or(Err("Error inserting time into database"))?;

    let tickets: Vec<TicketTime> = args.tickets.iter()
        .map(|s| Ticket::from_str(s).unwrap_or_else(|e| panic!("{e}")))
        .zip(iter::repeat(id))
        .map(TicketTime::from)
        .collect();

    insert_into(ticket_time::table)
        .values(tickets)
        .execute(conn)
        .or(Err("Error inserting ticket-time relations into database"))?;

    println!("Time logged successfully");

    Ok(())
}
use diesel::{prelude::*, query_builder::AsQuery};
use tabled::{Table, settings::Style};

use crate::{cli::args::{DocIdentifier, EntryType, ListArgs}, csv::convert::CsvTime, orm::{model::{Invoice, Time}, query::TimeWithTickets}, util::error::DynResult};


pub fn list(conn: &mut SqliteConnection, args: ListArgs) -> DynResult<()> {
    let ident = if args.all {
        args.ident
    } else {
        Some(args.ident.unwrap_or_default())
    };

    match args.entry_type {
        EntryType::Time => list_time(conn, ident),
        EntryType::Activity => todo!(),
        EntryType::Invoice => todo!(),
    }
}

pub fn list_time(conn: &mut SqliteConnection, ident: Option<DocIdentifier>) -> DynResult<()> {
    use crate::orm::schema::{invoice_activity, time};

    let times = if let Some(ident) = ident {
        let invoice = Invoice::select_by_identifier(ident, conn)?;

        TimeWithTickets::from_query(
            Time::query()
                .inner_join(invoice_activity::table)
                .filter(invoice_activity::inv_num.eq(invoice.inv_num)),
            conn
        )
    } else {
        TimeWithTickets::from_query(time::table.as_query(), conn)
    }.map_err(|e| format!("Error retrieving times from database:\n{e}"))?;

    println!("{}", Table::new(
        times.into_iter().map(CsvTime::from)
    ).with(Style::psql()));
    
    Ok(())
}
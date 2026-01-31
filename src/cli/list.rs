use std::error::Error;

use diesel::SqliteConnection;

use crate::cli::args::{EntryType, ListArgs};


pub fn list(conn: &mut SqliteConnection, args: ListArgs) -> Result<(), Box<dyn Error>> {
    dbg!(&args);
    match args.entry_type {
        EntryType::Time => list_time(conn),
        EntryType::Activity => todo!(),
        EntryType::Invoice => todo!(),
    }
}

pub fn list_time(conn: &mut SqliteConnection) -> Result<(), Box<dyn Error>> {
    use crate::orm::schema::{ticket_time, time};
    todo!()
}
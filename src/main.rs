use std::{env, error::Error};

use clap::Parser;
use diesel::{Connection, RunQueryDsl, SqliteConnection, sql_query};
use time_tracker::cli::{args::{Action, CliArgs}, generate, log};

fn main() {
    let args = CliArgs::parse();

    let db_url = args.database.unwrap_or_else(
        || env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set when no database argument is provided")
    );

    let conn = &mut SqliteConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url));

    // Need to enable foreign key checks for the session every time.
    sql_query("PRAGMA foreign_keys = ON")
        .execute(conn)
        .expect("Unable to enable foreign keys for database session");

    conn.transaction::<(), Box<dyn Error>, _>(|conn| match args.action {
        Action::Generate(gen_args) => generate::generate(conn, gen_args),
        Action::Log(log_args) => log::log(conn, log_args),
        Action::Amend(_) => todo!(),
        Action::List => todo!(),
    }).unwrap_or_else(|e| panic!("{e}"));
}
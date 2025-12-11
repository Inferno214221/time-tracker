use std::env;

use clap::Parser;
use diesel::{Connection, SqliteConnection};
use invoice_generator::cli::{args::{Action, Args}, generate};

fn main() {
    let args = Args::parse();

    let db_url = args.database.unwrap_or_else(
        || env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set when no database argument is provided")
    );

    let conn = &mut SqliteConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url));

    match args.action {
        Action::Generate {
            doc_type,
            ident,
            output
        } => generate::generate(conn, doc_type, ident, output),
        Action::Log {
            date,
            time_range,
            description,
            tickets
        } => todo!(),
        Action::Amend {
            // TODO
        } => todo!(),
    }
}
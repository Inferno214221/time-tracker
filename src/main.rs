use std::env;

use clap::Parser;
use diesel::{Connection, SqliteConnection};
use invoice_generator::cli::{args::{Action, CliArgs}, generate, log};

fn main() {
    let args = CliArgs::parse();

    let db_url = args.database.unwrap_or_else(
        || env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set when no database argument is provided")
    );

    let conn = &mut SqliteConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url));

    match args.action {
        Action::Generate(gen_args) => generate::generate(conn, gen_args),
        Action::Log(log_args) => log::log(conn, log_args),
        Action::Amend => todo!(),
    }
}
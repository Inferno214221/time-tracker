use std::error::Error;

use diesel::SqliteConnection;

use crate::cli::args::AmendArgs;

pub fn amend(conn: &mut SqliteConnection, args: AmendArgs) -> Result<(), Box<dyn Error>> {
    dbg!(&args);
    if args.delete {
        if !args.property.is_empty() {
            todo!()
        }
        todo!()
    } else {
        todo!()
    }
    Ok(())
}
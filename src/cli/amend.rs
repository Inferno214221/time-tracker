use diesel::SqliteConnection;

use crate::{cli::args::AmendArgs, util::error::DynResult};

pub fn amend(conn: &mut SqliteConnection, args: AmendArgs) -> DynResult<()> {
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
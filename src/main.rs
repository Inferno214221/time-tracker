use std::{env, fs};

use diesel::{Connection, HasQuery, SqliteConnection};
use invoice_generator::{orm::{model::InvoiceActivity, query::ActivityWithTickets}, typst::world::MinimalWorld};
use typst::{Library, LibraryExt};
use typst_pdf::PdfOptions;

fn main() {
    let lib = Library::builder().build();

    let world = MinimalWorld::new("../", "= Hello, World!", lib);

    let document = typst::compile(&world)
        .output
        .expect("Error compiling typst");

    let pdf = typst_pdf::pdf(&document, &PdfOptions::default()).expect("Error exporting PDF");
    fs::write("./out.pdf", pdf).expect("Error writing PDF");
    println!("Created pdf: `./output.pdf`");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = &mut SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    dbg!(
        ActivityWithTickets::from_query(InvoiceActivity::query(), conn)
        .unwrap()
        .sort_by_key(|i| i.activity.inv_num)
    );
}
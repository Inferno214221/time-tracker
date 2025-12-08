use std::{env, fs};

use diesel::{Connection, HasQuery, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};
use invoice_generator::{orm::{model::{Invoice, InvoiceActivity, Recipient}, query::ActivityWithTickets}, typst::world::MinimalWorld};
use typst::{Library, LibraryExt};
use typst_pdf::PdfOptions;

fn main() {
    let lib = Library::builder().build();

    let world = MinimalWorld::new("../", include_str!("../res/template.typ"), lib);

    let document = typst::compile(&world)
        .output
        .expect("Error compiling typst");

    let pdf = typst_pdf::pdf(&document, &PdfOptions::default()).expect("Error exporting PDF");
    fs::write("./out.pdf", pdf).expect("Error writing PDF");
    println!("Created pdf: './out.pdf'");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = &mut SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    let mut res = ActivityWithTickets::from_query(InvoiceActivity::query(), conn).unwrap();
    res.sort_by_key(|i| i.inv_num);

    dbg!(res);

    use invoice_generator::orm::schema::{invoice, recipient};

    dbg!(
        invoice::table
            .inner_join(recipient::table)
            .select((Invoice::as_select(), Recipient::as_select()))
            .load::<(Invoice, Recipient)>(conn)
    );
}
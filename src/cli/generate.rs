use std::{fs, path::PathBuf};

use chrono::{Datelike, Local};
use diesel::{prelude::*};
use typst::{Library, LibraryExt};
use typst_pdf::PdfOptions;

use crate::{cli::args::{DocIdentifier, DocType}, orm::{model::{Invoice, Recipient}, query::InvoiceWithActivities}, typst::{convert::IntoTypst, world::MinimalWorld}};

pub fn generate(
    conn: &mut SqliteConnection,
    doc_type: DocType,
    ident: Option<DocIdentifier>,
    output: Option<PathBuf>
) {
    let ident = ident.unwrap_or_else(|| DocIdentifier::Month(
        Local::now().date_naive()
    ));
    match doc_type {
        DocType::Invoice => generate_invoice(conn, ident, output),
        DocType::Timesheet => generate_timesheet(conn, ident, output),
    }
}

pub fn generate_invoice(
    conn: &mut SqliteConnection,
    ident: DocIdentifier,
    output: Option<PathBuf>
) {
    use crate::orm::schema::{invoice, recipient};

    let invoices = match ident {
        DocIdentifier::Num(n) => InvoiceWithActivities::from_query(
            invoice::table
                .inner_join(recipient::table)
                .filter(invoice::inv_num.eq(n))
                .select((Invoice::as_select(), Recipient::as_select())),
            conn
        ),
        DocIdentifier::Month(m) => InvoiceWithActivities::from_query(
            invoice::table
                .inner_join(recipient::table)
                .filter(invoice::inv_month.eq(m))
                .select((Invoice::as_select(), Recipient::as_select())),
            conn
        ),
    }.expect("Error retrieving invoice from database");

    let Ok([invoice]) = <[InvoiceWithActivities; 1]>::try_from(invoices) else {
        panic!("Identifier failed to uniquely identify an invoice");
    };

    // TODO: inv_created.unwrap_or_else(|| Local::now().date_naive()) should probably occur
    // somewhere here.

    let output = output.unwrap_or_else(
        || format!(
            "./{}-{}-tax-invoice-{}.pdf",
            invoice.inv_month.year(),
            invoice.inv_month.month(),
            invoice.inv_num
        ).into()
    );
    
    let lib = Library::builder()
        .with_inputs(invoice.into_typst())
        .build();

    let world = MinimalWorld::new("../", include_str!("../../res/template.typ"), lib);

    let document = typst::compile(&world)
        .output
        .expect("Error compiling typst template");

    let pdf = typst_pdf::pdf(&document, &PdfOptions::default())
        .expect("Error exporting PDF");

    fs::write(&output, pdf)
        .expect("Error writing PDF");

    println!("Created invoice: '{}'", output.display());
}

pub fn generate_timesheet(
    conn: &mut SqliteConnection,
    ident: DocIdentifier,
    output: Option<PathBuf>
) {
    //
}
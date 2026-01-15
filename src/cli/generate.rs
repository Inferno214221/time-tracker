use std::{fs, path::PathBuf};

use chrono::{Datelike, Local};
use csv::{QuoteStyle, WriterBuilder};
use diesel::prelude::*;
use typst::{Library, LibraryExt};
use typst_pdf::PdfOptions;

use crate::cli::args::{DocIdentifier, DocType, GenerateArgs};
use crate::csv::convert::CsvTime;
use crate::orm::model::Time;
use crate::orm::query::TimeWithTickets;
use crate::orm::{model::{Invoice, Recipient}, query::InvoiceWithActivities};
use crate::typst::{convert::IntoTypst, world::MinimalWorld};

pub fn generate(conn: &mut SqliteConnection, args: GenerateArgs) {
    let ident = args.ident.unwrap_or_else(|| DocIdentifier::Month(
        Local::now().date_naive()
    ));

    match args.doc_type {
        DocType::Invoice => generate_invoice(conn, ident, args.output),
        DocType::Timesheet => generate_timesheet(conn, ident, args.output),
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
    use crate::orm::schema::{invoice, invoice_activity};

    let invoices = match ident {
        DocIdentifier::Num(n) => Invoice::query()
            .filter(invoice::inv_num.eq(n))
            .load(conn),
        DocIdentifier::Month(m) => Invoice::query()
            .filter(invoice::inv_month.eq(m))
            .load(conn),
    }.expect("Error retrieving timesheet from database");

    let Ok([invoice]) = <[Invoice; 1]>::try_from(invoices) else {
        panic!("Identifier failed to uniquely identify an timesheet");
    };

    let times = TimeWithTickets::from_query(
        Time::query()
            .inner_join(invoice_activity::table)
            .filter(invoice_activity::inv_num.eq(invoice.inv_num)),
        conn
    ).expect("Error retrieving timesheet from database");

    let output = output.unwrap_or_else(
        || format!(
            "./{}-{}-timesheet-{}.csv",
            invoice.inv_month.year(),
            invoice.inv_month.month(),
            invoice.inv_num
        ).into()
    );

    let mut writer = WriterBuilder::new()
        .quote_style(QuoteStyle::Always)
        .has_headers(false)
        .from_path(&output)
        .unwrap_or_else(|_| panic!("Error opening file {}", output.display()));

    // Just manually write the headers so that they are pretty.
    writer.write_record(["Start", "End", "Duration", "Tickets", "Description"])
        .expect("Error writing time entry to timesheet");

    for time in times {
        writer.serialize(CsvTime::from(time))
            .expect("Error writing time entry to timesheet");
    }

    println!("Created timesheet: '{}'", output.display());
}
use std::error::Error;
use std::{fs, path::PathBuf};

use chrono::{Datelike, Local, NaiveDate};
use csv::{QuoteStyle, WriterBuilder};
use diesel::prelude::*;
use typst::{Library, LibraryExt};
use typst_pdf::PdfOptions;

use crate::cli::args::{DocIdentifier, DocType, GenerateArgs};
use crate::csv::convert::CsvTime;
use crate::orm::model::Time;
use crate::orm::query::TimeWithTickets;
use crate::orm::{model::{Invoice, Recipient}, query::InvoiceWithActivities};
use crate::typst::error::DisplayErrors;
use crate::typst::{convert::IntoTypst, world::MinimalWorld};

pub fn generate(conn: &mut SqliteConnection, args: GenerateArgs) -> Result<(), Box<dyn Error>> {
    let now = Local::now().date_naive();
    let ident = args.ident.unwrap_or_else(|| DocIdentifier::Month(
        NaiveDate::from_ymd_opt(now.year(), now.month(), 1)
            .expect("Failed to reconstruct date from parts")
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
) -> Result<(), Box<dyn Error>> {
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
    }.map_err(|e| format!("Error retrieving invoice from database:\n{e}"))?;

    let Ok([invoice]) = <[InvoiceWithActivities; 1]>::try_from(invoices) else {
        return Err("Identifier failed to uniquely identify an invoice".into());
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
        .map_err(|e| format!("Error compiling typst template:{}", DisplayErrors(e)))?;

    let pdf = typst_pdf::pdf(&document, &PdfOptions::default())
        .map_err(|e| format!("Error exporting PDF:{}", DisplayErrors(e)))?;

    fs::write(&output, pdf)
        .map_err(|e| format!("Error writing PDF:\n{e}"))?;

    println!("Created invoice: '{}'", output.display());

    Ok(())
}

pub fn generate_timesheet(
    conn: &mut SqliteConnection,
    ident: DocIdentifier,
    output: Option<PathBuf>
) -> Result<(), Box<dyn Error>> {
    use crate::orm::schema::{invoice, invoice_activity};

    let invoices = match ident {
        DocIdentifier::Num(n) => Invoice::query()
            .filter(invoice::inv_num.eq(n))
            .load(conn),
        DocIdentifier::Month(m) => Invoice::query()
            .filter(invoice::inv_month.eq(m))
            .load(conn),
    }.map_err(|e| format!("Error retrieving invoice from database:\n{e}"))?;

    let Ok([invoice]) = <[Invoice; 1]>::try_from(invoices) else {
        return Err("Identifier failed to uniquely identify an timesheet".into());
    };

    let times = TimeWithTickets::from_query(
        Time::query()
            .inner_join(invoice_activity::table)
            .filter(invoice_activity::inv_num.eq(invoice.inv_num)),
        conn
    ).map_err(|e| format!("Error retrieving timesheet from database:\n{e}"))?;

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
        .map_err(|e| format!("Error opening file {}:\n{e}", output.display()))?;

    // Just manually write the headers so that they are pretty.
    writer.write_record(["Start", "End", "Duration", "Tickets", "Description"])
        .map_err(|e| format!("Error writing time entry to timesheet:\n{e}"))?;

    for time in times {
        writer.serialize(CsvTime::from(time))
            .map_err(|e| format!("Error writing time entry to timesheet:\n{e}"))?;
    }

    println!("Created timesheet: '{}'", output.display());

    Ok(())
}
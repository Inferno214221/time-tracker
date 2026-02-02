use std::{fs, path::PathBuf};

use csv::{QuoteStyle, WriterBuilder};
use diesel::prelude::*;
use typst::{Library, LibraryExt};
use typst_pdf::PdfOptions;

use crate::cli::args::{DocIdentifier, DocType, GenerateArgs};
use crate::csv::convert::CsvTime;
use crate::orm::model::Time;
use crate::orm::query::TimeWithTickets;
use crate::orm::{model::Invoice, query::InvoiceWithActivities};
use crate::typst::error::DisplayErrors;
use crate::typst::{convert::IntoTypst, world::MinimalWorld};
use crate::util::error::DynResult;

pub fn generate(conn: &mut SqliteConnection, args: GenerateArgs) -> DynResult<()> {
    let ident = args.ident.unwrap_or_default();

    match args.doc_type {
        DocType::Invoice => generate_invoice(conn, ident, args.output),
        DocType::Timesheet => generate_timesheet(conn, ident, args.output),
    }
}

pub fn generate_invoice(
    conn: &mut SqliteConnection,
    ident: DocIdentifier,
    output: Option<PathBuf>
) -> DynResult<()> {
    let invoice = InvoiceWithActivities::select_by_identifier(ident, conn)?;

    let output = output.unwrap_or_else(
        || format!(
            "./{}-tax-invoice-{}.pdf",
            invoice.inv_month,
            invoice.inv_num
        ).into()
    );
    
    let lib = Library::builder()
        .with_inputs(invoice.into_typst())
        .build();

    let world = MinimalWorld::new("../", include_str!("../../res/template.typ"), lib);

    let document = typst::compile(&world)
        .output
        .map_err(|e| format!("Error compiling typst template:\n{}", DisplayErrors(e)))?;

    let pdf = typst_pdf::pdf(&document, &PdfOptions::default())
        .map_err(|e| format!("Error exporting PDF:\n{}", DisplayErrors(e)))?;

    fs::write(&output, pdf)
        .map_err(|e| format!("Error writing PDF:\n{e}"))?;

    println!("Created invoice: '{}'", output.display());

    Ok(())
}

pub fn generate_timesheet(
    conn: &mut SqliteConnection,
    ident: DocIdentifier,
    output: Option<PathBuf>
) -> DynResult<()> {
    use crate::orm::schema::invoice_activity;

    let invoice = Invoice::select_by_identifier(ident, conn)?;

    let times = TimeWithTickets::from_query(
        Time::query()
            .inner_join(invoice_activity::table)
            .filter(invoice_activity::inv_num.eq(invoice.inv_num)),
        conn
    ).map_err(|e| format!("Error retrieving timesheet from database:\n{e}"))?;

    let output = output.unwrap_or_else(
        || format!(
            "./{}-timesheet-{}.csv",
            invoice.inv_month,
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
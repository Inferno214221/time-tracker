use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};
use typst::foundations::{Array, Datetime, Dict, IntoValue, Str, Value};

use crate::orm::{model::Recipient, query::{ActivityWithTickets, InvoiceWithActivities}, ticket::Ticket};

// IntoValue exists, but the result is always a Value (enum) and the orphan rule prevents me from
// implementing it for chrono types.
pub trait IntoTypst {
    type Output;

    fn into_typst(self) -> Self::Output;
}

impl IntoTypst for Ticket {
    type Output = Str;

    fn into_typst(self) -> Self::Output {
        Str::from(format!("{}", self))
    }
}

impl IntoTypst for Recipient {
    type Output = Dict;

    fn into_typst(self) -> Self::Output {
        [
            (Str::from("name"), Str::from(self.recip_name).into_value()),
            (Str::from("addr"), Str::from(self.recip_addr.replace("\\n", "\n")).into_value()),
        ].into_iter().collect()
    }
}

impl IntoTypst for ActivityWithTickets {
    type Output = Dict;

    fn into_typst(self) -> Self::Output {
        [
            (Str::from("desc"), Str::from(self.act_desc).into_value()),
            (Str::from("uprice"), self.act_uprice.into_value()),
            (Str::from("dur"), self.act_dur.into_value()),
            (Str::from("tickets"), self.tickets.into_iter()
                .map(|t| Value::Str(t.into_typst()))
                .collect::<Array>()
                .into_value()
            )
        ].into_iter().collect()
    }
}

impl IntoTypst for InvoiceWithActivities {
    type Output = Dict;

    fn into_typst(self) -> Self::Output {
        [
            (Str::from("num"), self.inv_num.into_value()),
            (Str::from("month"), self.inv_month.into_typst().into_value()),
            (Str::from("created"), self.inv_created.unwrap_or_default()
                .into_typst()
                .into_value()
            ),
            (Str::from("recipient"), self.recipient.into_typst().into_value()),
            (Str::from("activities"), self.activities.into_iter()
                .map(|t| t.into_typst().into_value())
                .collect::<Array>()
                .into_value()
            )
        ].into_iter().collect()
    }
}

impl IntoTypst for NaiveDateTime {
    type Output = Datetime;

    fn into_typst(self) -> Self::Output {
        Datetime::from_ymd_hms(
            self.year(),
            self.month() as u8,
            self.day() as u8,
            self.hour() as u8,
            self.minute() as u8,
            self.second() as u8
        ).expect("Date should already be valid.")
    }
}

impl IntoTypst for NaiveDate {
    type Output = Datetime;

    fn into_typst(self) -> Self::Output {
        Datetime::from_ymd(
            self.year(),
            self.month() as u8,
            self.day() as u8
        ).expect("Date should already be valid.")
    }
}
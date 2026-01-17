use chrono::{NaiveDate, NaiveDateTime};
use derive_more::{Debug, Display};
use diesel::prelude::*;
use diesel::sqlite::Sqlite;

use super::schema;

#[derive(Debug, HasQuery, Identifiable)]
#[diesel(table_name = schema::project)]
#[diesel(primary_key(proj_key))]
#[diesel(check_for_backend(Sqlite))]
pub struct Project {
    pub proj_key: String,
    pub proj_name: String,
}

#[derive(Debug, HasQuery, Identifiable, Associations)]
#[derive(Display, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(belongs_to(Project, foreign_key = proj_key))]
#[diesel(table_name = schema::ticket)]
#[diesel(primary_key(proj_key, tick_num))]
#[diesel(check_for_backend(Sqlite))]
#[debug("\"{proj_key}-{tick_num}\"")]
#[display("{proj_key}-{tick_num}")]
pub struct Ticket {
    pub proj_key: String,
    pub tick_num: i32,
}

#[derive(Debug, HasQuery, Identifiable, Associations, Insertable)]
#[diesel(belongs_to(Project, foreign_key = proj_key))]
// #[diesel(belongs_to(Ticket))]
#[diesel(belongs_to(Time, foreign_key = time_id))]
#[diesel(table_name = schema::ticket_time)]
#[diesel(primary_key(proj_key, tick_num, time_id))]
#[diesel(check_for_backend(Sqlite))]
pub struct TicketTime {
    pub proj_key: String,
    pub tick_num: i32,
    pub time_id: i32,
}

impl From<(Ticket, i32)> for TicketTime {
    fn from((ticket, time_id): (Ticket, i32)) -> Self {
        TicketTime {
            proj_key: ticket.proj_key,
            tick_num: ticket.tick_num,
            time_id
        }
    }
}

#[derive(Debug, HasQuery, Identifiable, Associations)]
#[diesel(belongs_to(InvoiceActivity, foreign_key = act_num))]
#[diesel(table_name = schema::time)]
#[diesel(primary_key(time_id))]
#[diesel(check_for_backend(Sqlite))]
pub struct Time {
    pub time_id: i32,
    pub time_start: NaiveDateTime,
    pub time_end: NaiveDateTime,
    pub time_desc: String,
    pub time_dur: Option<f64>,
    pub act_num: Option<i32>,
}

#[derive(Debug, HasQuery, Identifiable)]
#[diesel(table_name = schema::recipient)]
#[diesel(primary_key(recip_id))]
#[diesel(check_for_backend(Sqlite))]
pub struct Recipient {
    pub recip_id: String,
    pub recip_name: String,
    pub recip_addr: String,
}

#[derive(Debug, HasQuery, Identifiable, Associations)]
#[diesel(belongs_to(Recipient, foreign_key = recip_id))]
#[diesel(table_name = schema::invoice)]
#[diesel(primary_key(inv_num))]
#[diesel(check_for_backend(Sqlite))]
pub struct Invoice {
    pub inv_num: i32,
    pub inv_month: NaiveDate,
    pub inv_created: Option<NaiveDate>,
    pub recip_id: String,
}

#[derive(Debug, HasQuery, Identifiable, Associations)]
#[diesel(belongs_to(Invoice, foreign_key = inv_num))]
#[diesel(table_name = schema::invoice_activity)]
#[diesel(primary_key(act_num))]
#[diesel(check_for_backend(Sqlite))]
pub struct InvoiceActivity {
    pub act_num: i32,
    pub inv_num: i32,
    pub act_desc: String,
    pub act_uprice: f64,
}
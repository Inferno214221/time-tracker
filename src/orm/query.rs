use std::collections::BTreeSet;

use diesel::prelude::*;
use diesel::query_dsl::methods::LoadQuery;

use crate::{cli::args::DocIdentifier, orm::{model::{Invoice, InvoiceActivity, Recipient, TicketTime, Time}, ticket::Ticket}, util::{date::{Date, DateTime, Month}, error::DynResult}};
use super::schema;

#[derive(Debug, Identifiable, Associations)]
#[diesel(belongs_to(InvoiceActivity, foreign_key = act_num))]
#[diesel(table_name = schema::time)]
#[diesel(primary_key(time_id))]
#[diesel(check_for_backend(Sqlite))]
pub struct TimeWithTickets {
    pub time_id: i32,
    pub time_start: DateTime,
    pub time_end: DateTime,
    pub time_desc: String,
    pub time_dur: Option<f64>,
    pub act_num: Option<i32>,
    pub tickets: Vec<Ticket>,
}

impl From<(Vec<TicketTime>, Time)> for TimeWithTickets {
    fn from((tickets, time): (Vec<TicketTime>, Time)) -> Self {
        TimeWithTickets {
            time_id: time.time_id,
            time_start: time.time_start,
            time_end: time.time_end,
            time_desc: time.time_desc,
            time_dur: time.time_dur,
            act_num: time.act_num,
            tickets: tickets.into_iter()
                .map(Ticket::from)
                .collect(),
        }
    }
}

impl TimeWithTickets {
    pub fn from_query<'q, Q>(
        query: Q,
        conn: &mut SqliteConnection
    ) -> QueryResult<Vec<TimeWithTickets>> where
        Q: LoadQuery<'q, SqliteConnection, Time>
    {
        let all_times = query.load(conn)?;
        
        let all_tickets: Vec<TicketTime> = TicketTime::belonging_to(&all_times)
            .load(conn)?;

        Ok(all_tickets.grouped_by(&all_times)
            .into_iter()
            .zip(all_times)
            .map(TimeWithTickets::from)
            .collect())
    }
}

#[derive(Debug, Identifiable, Associations)]
#[diesel(belongs_to(Invoice, foreign_key = inv_num))]
#[diesel(table_name = schema::invoice)]
#[diesel(primary_key(act_num))]
#[diesel(check_for_backend(Sqlite))]
pub struct ActivityWithTickets {
    pub act_num: i32,
    pub inv_num: i32,
    pub act_desc: String,
    pub act_uprice: f64,
    pub act_dur: f64,
    pub tickets: BTreeSet<Ticket>,
}

impl From<(Vec<TimeWithTickets>, InvoiceActivity)> for ActivityWithTickets {
    fn from((time_with_tickets, activity): (Vec<TimeWithTickets>, InvoiceActivity)) -> Self {
        ActivityWithTickets {
            act_num: activity.act_num,
            inv_num: activity.inv_num,
            act_desc: activity.act_desc,
            act_uprice: activity.act_uprice,
            act_dur: time_with_tickets.iter()
                .flat_map(|t| t.time_dur)
                .sum(),
            tickets: time_with_tickets.into_iter()
                .flat_map(|t| t.tickets)
                .collect()
        }
    }
}

impl ActivityWithTickets {
    pub fn from_query<'q, Q>(
        query: Q,
        conn: &mut SqliteConnection
    ) -> QueryResult<Vec<ActivityWithTickets>> where
        Q: LoadQuery<'q, SqliteConnection, InvoiceActivity>
    {
        let all_activities = query.load(conn)?;

        let times_with_tickets = TimeWithTickets::from_query(
            Time::belonging_to(&all_activities),
            conn
        )?;

        Ok(times_with_tickets.grouped_by(&all_activities)
            .into_iter()
            .zip(all_activities)
            .map(ActivityWithTickets::from)
            .collect())
    }
}

#[derive(Debug)]
pub struct InvoiceWithActivities {
    pub inv_num: i32,
    pub inv_month: Month,
    pub inv_created: Option<Date>,
    pub recipient: Recipient,
    pub activities: Vec<ActivityWithTickets>,
}

impl From<(Vec<ActivityWithTickets>, Invoice, Recipient)> for InvoiceWithActivities {
    fn from((
        mut activities,
        invoice,
        recipient
    ): (
        Vec<ActivityWithTickets>,
        Invoice,
        Recipient
    )) -> Self {
        activities.sort_by_key(|a| a.act_num);
        InvoiceWithActivities {
            inv_num: invoice.inv_num,
            inv_month: invoice.inv_month,
            inv_created: invoice.inv_created,
            recipient,
            activities,
        }
    }
}

impl InvoiceWithActivities {
    pub fn from_query<'q, Q>(
        query: Q,
        conn: &mut SqliteConnection
    ) -> QueryResult<Vec<InvoiceWithActivities>> where
        Q: LoadQuery<'q, SqliteConnection, (Invoice, Recipient)>
    {
        let (all_invoices, recipients): (Vec<_>, Vec<_>) = query.load(conn)?
            .into_iter()
            .unzip();

        let activities_with_tickets = ActivityWithTickets::from_query(
            InvoiceActivity::belonging_to(&all_invoices),
            conn
        )?;

        Ok(activities_with_tickets.grouped_by(&all_invoices)
            .into_iter()
            .zip(all_invoices)
            .zip(recipients)
            .map(|((a, b), c)| (a, b, c))
            .map(InvoiceWithActivities::from)
            .collect())
    }

    pub fn select_by_identifier(
        ident: DocIdentifier,
        conn: &mut SqliteConnection
    ) -> DynResult<InvoiceWithActivities> {
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

        let [invoice] = <[InvoiceWithActivities; 1]>::try_from(invoices)
            .map_err(|_| "Identifier failed to uniquely identify an invoice")?;

        Ok(invoice)
    }
}

impl Invoice {
    pub fn select_by_identifier(
        ident: DocIdentifier,
        conn: &mut SqliteConnection
    ) -> DynResult<Invoice> {
        use crate::orm::schema::{invoice};

        let invoices = match ident {
            DocIdentifier::Num(n) => Invoice::query()
                .filter(invoice::inv_num.eq(n))
                .load(conn),
            DocIdentifier::Month(m) => Invoice::query()
                .filter(invoice::inv_month.eq(m))
                .load(conn),
        }.map_err(|e| format!("Error retrieving invoice from database:\n{e}"))?;

        let [invoice] = <[Invoice; 1]>::try_from(invoices)
            .map_err(|_| "Identifier failed to uniquely identify an invoice")?;

        Ok(invoice)
    }
}
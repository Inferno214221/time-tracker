use std::collections::BTreeSet;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::query_dsl::methods::LoadQuery;

use crate::orm::model::{InvoiceActivity, Ticket, TicketTime, Time};
use super::schema;

#[derive(Debug, Identifiable, Associations)]
#[diesel(belongs_to(InvoiceActivity, foreign_key = act_num))]
#[diesel(table_name = schema::time)]
#[diesel(primary_key(time_id))]
#[diesel(check_for_backend(Sqlite))]
pub struct TimeWithTickets {
    pub time_id: i32,
    pub time_start: NaiveDateTime,
    pub time_end: NaiveDateTime,
    pub time_desc: String,
    pub time_dur: Option<f64>,
    pub act_num: Option<i32>,
    pub tickets: Vec<Ticket>,
}

impl From<(Vec<TicketTime>, Time)> for TimeWithTickets {
    fn from(value: (Vec<TicketTime>, Time)) -> Self {
        let (tickets, time) = value;
        TimeWithTickets {
            time_id: time.time_id,
            time_start: time.time_start,
            time_end: time.time_end,
            time_desc: time.time_desc,
            time_dur: time.time_dur,
            act_num: time.act_num,
            tickets: tickets.into_iter().map(|t| t.into()).collect(),
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

#[derive(Debug)]
pub struct ActivityWithTickets {
    pub activity: InvoiceActivity,
    pub tickets: BTreeSet<Ticket>,
    pub duration: f64,
}

impl ActivityWithTickets {
    pub fn from_query<'q, Q>(
        query: Q,
        conn: &mut SqliteConnection
    ) -> QueryResult<Vec<ActivityWithTickets>> where
        Q: LoadQuery<'q, SqliteConnection, InvoiceActivity>
    {
        let all_activites = query.load(conn)?;

        let all_times: Vec<Time> = Time::belonging_to(&all_activites)
            .load(conn)?;

        let all_tickets: Vec<TicketTime> = TicketTime::belonging_to(&all_times)
            .load(conn)?;

        let times_with_tickets: Vec<TimeWithTickets> = all_tickets.grouped_by(&all_times)
            .into_iter()
            .zip(all_times)
            .map(TimeWithTickets::from)
            .collect();

        Ok(times_with_tickets.grouped_by(&all_activites)
            .into_iter()
            .zip(all_activites)
            .map(|(time_with_tickets, activity)| ActivityWithTickets {
                activity,
                duration: time_with_tickets.iter()
                    .flat_map(|t| t.time_dur)
                    .sum(),
                tickets: time_with_tickets.into_iter()
                    .flat_map(|t| t.tickets)
                    .collect()
            }).collect())
    }
}
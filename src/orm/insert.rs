use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::orm::schema;

#[derive(Debug, Insertable)]
#[diesel(table_name = schema::time)]
pub struct LoggedTime {
    pub time_start: NaiveDateTime,
    pub time_end: NaiveDateTime,
    pub time_desc: String,
    pub act_num: i32,
}
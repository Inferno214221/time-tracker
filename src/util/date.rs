use std::{fmt::{self, Display, Formatter}, ops::Deref, str::FromStr};

use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, Timelike};
use diesel::{Queryable, backend::Backend, deserialize::{self, FromSql}, expression::AsExpression, sqlite::Sqlite};
use diesel::sql_types::{Date as SqlDate, Timestamp};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Month(NaiveDate);

impl Month {
    pub fn current() -> Month {
        let now = Local::now().date_naive();
        Month(NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap())
    }
}

impl Default for Month {
    fn default() -> Self {
        Month::current()
    }
}

impl Deref for Month {
    type Target = NaiveDate;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Month {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Month(
            NaiveDate::parse_from_str(&(s.to_owned() + "-01"), "%Y-%m-%d")
                .map_err(|e| format!("Error parsing month:\n{e}"))?
        ))
    }
}

impl Display for Month {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:0>4}-{:0>2}",
            self.0.year(),
            self.0.month()
        )
    }
}

impl AsExpression<SqlDate> for Month {
    type Expression = <NaiveDate as AsExpression<SqlDate>>::Expression;

    fn as_expression(self) -> Self::Expression {
        AsExpression::<SqlDate>::as_expression(self.0)
    }
}

impl Queryable<SqlDate, Sqlite> for Month {
    type Row = NaiveDate;

    fn build(row: Self::Row) -> deserialize::Result<Self> {
        Ok(Month(row))
    }
}

impl FromSql<SqlDate, Sqlite> for Month {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        Ok(Month(NaiveDate::from_sql(bytes)?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Date(NaiveDate);

impl Date {
    pub fn now() -> Date {
        Date(Local::now().date_naive())
    }
}

impl Default for Date {
    fn default() -> Self {
        Date::now()
    }
}

impl Deref for Date {
    type Target = NaiveDate;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Date {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Date(
            NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .map_err(|e| format!("Error parsing date:\n{e}"))?
        ))
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:0>4}-{:0>2}-{:0>2}",
            self.0.year(),
            self.0.month(),
            self.0.day()
        )
    }
}

impl AsExpression<SqlDate> for Date {
    type Expression = <NaiveDate as AsExpression<SqlDate>>::Expression;

    fn as_expression(self) -> Self::Expression {
        AsExpression::<SqlDate>::as_expression(self.0)
    }
}

impl Queryable<SqlDate, Sqlite> for Date {
    type Row = NaiveDate;

    fn build(row: Self::Row) -> deserialize::Result<Self> {
        Ok(Date(row))
    }
}

impl FromSql<SqlDate, Sqlite> for Date {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        Ok(Date(NaiveDate::from_sql(bytes)?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, AsExpression)]
#[diesel(sql_type = SqlDate)]
pub struct DateTime(NaiveDateTime);

impl Deref for DateTime {
    type Target = NaiveDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:0>4}-{:0>2}-{:0>2} {:0>2}:{:0>2}",
            self.0.year(),
            self.0.month(),
            self.0.day(),
            self.0.hour(),
            self.0.minute()
        )
    }
}

impl Queryable<Timestamp, Sqlite> for DateTime {
    type Row = NaiveDateTime;

    fn build(row: Self::Row) -> deserialize::Result<Self> {
        Ok(DateTime(row))
    }
}

impl FromSql<Timestamp, Sqlite> for DateTime {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        Ok(DateTime(<NaiveDateTime as FromSql<Timestamp, Sqlite>>::from_sql(bytes)?))
    }
}
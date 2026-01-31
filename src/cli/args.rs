use std::{path::PathBuf, str::FromStr};

use chrono::{NaiveTime, TimeDelta};
use clap::{Args, Parser, Subcommand, builder::styling::Styles};

use crate::{cli::patterns::{ACTIVITY_PATTERN, DATE_PATTERN, TICKET_PATTERN, TIME_RANGE_PATTERN, TimeRangePatternCaptures}, orm::ticket::Ticket, util::date::{Date, Month}};

pub const CARGO_STYLES: Styles = {
    use clap_cargo::style::*;

    Styles::styled()
        .header(HEADER)
        .usage(USAGE)
        .literal(LITERAL)
        .placeholder(PLACEHOLDER)
        .error(ERROR)
        .valid(VALID)
        .invalid(INVALID)
};

// TODO: Write documentation about cli interface

#[derive(Debug, Parser)]
#[command(version, about, styles = CARGO_STYLES)]
pub struct CliArgs {
    #[command(subcommand)]
    pub action: Action,

    #[arg(long, short = 'i', global = true)]
    pub database: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    #[command(visible_alias = "gen")]
    Generate(GenerateArgs),
    Log(LogArgs),
    Amend(AmendArgs),
    #[command(visible_alias = "ls")]
    List(ListArgs),
}

#[derive(Debug, Args)]
pub struct GenerateArgs {
    #[command(subcommand)]
    pub doc_type: DocType,

    #[arg(global = true, value_parser = DocIdentifier::from_str)]
    pub ident: Option<DocIdentifier>,

    #[arg(long, short, global = true)]
    pub output: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
pub enum DocType {
    #[command(visible_alias = "inv")]
    Invoice,
    #[command(visible_alias = "ts")]
    Timesheet,
}

#[derive(Debug, Clone)]
pub enum DocIdentifier {
    Num(i32),
    Month(Month),
}

impl Default for DocIdentifier {
    fn default() -> Self {
        DocIdentifier::Month(Month::default())
    }
}

impl FromStr for DocIdentifier {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = s.parse() {
            Ok(DocIdentifier::Num(num))
        } else if let Ok(date) = s.parse() {
            Ok(DocIdentifier::Month(date))
        } else {
            Err("Value doesn't match format of numeric id or month".into())
        }
    }
}

#[derive(Debug, Args)]
pub struct LogArgs {
    #[arg(long, short, value_parser = Date::from_str)]
    pub date: Option<Date>,

    #[arg(long, short)]
    pub activity: Option<i32>,

    #[arg(value_parser = TimeRange::from_str)]
    pub time_range: TimeRange,

    pub description: String,

    // TODO: Try changing to Ticket.
    #[arg(trailing_var_arg = true)]
    pub tickets: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TimeRange {
    pub start: NaiveTime,
    pub end: NaiveTime
}

impl TimeRange {
    pub fn try_from_match(groups: TimeRangePatternCaptures<'_>) -> Result<TimeRange, String> {
        let primary = NaiveTime::from_hms_opt(
            groups.hours.content.parse().unwrap(),
            groups.mins.map(|m| m.content).unwrap_or("0").parse().unwrap(),
            0
        ).ok_or("Invalid time provided")?;

        let offset = TimeDelta::minutes(
            (f32::from_str(groups.dur.content).unwrap() * 60_f32).round() as i64
        );

        if groups.op.content == "-" {
            Ok(TimeRange {
                start: primary - offset,
                end: primary,
            })
        } else {
            Ok(TimeRange {
                start: primary,
                end: primary + offset,
            })
        }
    }
}

impl FromStr for TimeRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let groups = TIME_RANGE_PATTERN.captures(s)
            .ok_or("Value doesn't match time range format")?;

        Self::try_from_match(groups)
    }
}

#[derive(Debug, Args)]
pub struct AmendArgs {
    // TODO: Need a method of showing the ids without manual access.
    #[arg(long, short)]
    pub time_id: Option<i32>,

    #[arg(long, short)]
    pub delete: bool,

    #[arg(value_parser = TimeProperty::from_str, trailing_var_arg = true)]
    pub property: Vec<TimeProperty>,
}

#[derive(Debug, Clone)]
pub enum TimeProperty {
    Date(Date),
    Time(TimeRange),
    Activity(i32),
    Ticket(Ticket),
    Desc(String),
}

// TODO: Test that this actually works.

impl FromStr for TimeProperty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Patterns are sorted by complexity.
        Ok(if ACTIVITY_PATTERN.is_match(s) {
            TimeProperty::Activity(
                s.parse()
                    .map_err(|e| format!("Error parsing activity number:\n{e}"))?
            )
        } else if let Some(groups) = TICKET_PATTERN.captures(s) {
            TimeProperty::Ticket(
                Ticket::try_from_match(groups)?
            )
        } else if DATE_PATTERN.is_match(s) {
            TimeProperty::Date(
                s.parse()?
            )
        } else if let Some(groups) = TIME_RANGE_PATTERN.captures(s) {
            TimeProperty::Time(
                TimeRange::try_from_match(groups)?
            )
        } else if !s.contains(' ') {
            Err("Value would be parsed as a description but contains no space.")?
        } else {
            TimeProperty::Desc(s.to_owned())
        })
    }
}

#[derive(Debug, Args)]
pub struct ListArgs {
    #[command(subcommand)]
    pub entry_type: EntryType,
}

#[derive(Debug, Clone, Subcommand)]
pub enum EntryType {
    Time,
    #[command(visible_alias = "act")]
    Activity,
    #[command(visible_alias = "inv")]
    Invoice,
}
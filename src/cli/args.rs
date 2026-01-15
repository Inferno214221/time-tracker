use std::{path::PathBuf, str::FromStr, sync::LazyLock};

use chrono::{NaiveDate, NaiveTime, TimeDelta};
use clap::{Args, Parser, Subcommand, builder::styling::Styles};
use ctreg::regex;

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
    Amend,
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

#[derive(Debug, Args)]
pub struct LogArgs {
    #[arg(long, short, value_parser = parse_date)]
    pub date: Option<NaiveDate>,

    #[arg(value_parser = TimeRange::from_str)]
    pub time_range: TimeRange,

    pub description: String,

    #[arg(trailing_var_arg = true)]
    pub tickets: Vec<String>,
}

#[derive(Debug, Subcommand)]
pub enum DocType {
    Invoice,
    Timesheet,
}

pub fn parse_month(input: &str) -> Result<NaiveDate, chrono::ParseError> {
    parse_date(&(input.to_owned() + "-01"))
}

pub fn parse_date(input: &str) -> Result<NaiveDate, chrono::ParseError> {
    NaiveDate::parse_from_str(input, "%Y-%m-%d")
}

#[derive(Debug, Clone)]
pub enum DocIdentifier {
    Num(i32),
    Month(NaiveDate),
}

impl FromStr for DocIdentifier {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = i32::from_str(s) {
            Ok(DocIdentifier::Num(num))
        } else if let Ok(date) = parse_month(s) {
            Ok(DocIdentifier::Month(date))
        } else {
            Err("value doesn't match format of numeric id or month".into())
        }
    }
}

#[derive(Debug, Clone)]
pub struct TimeRange {
    pub start: NaiveTime,
    pub end: NaiveTime
}

impl FromStr for TimeRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let groups = TIME_RANGE_PATTERN.captures(s).ok_or("value doesn't match time range format")?;

        let primary = NaiveTime::from_hms_opt(
            groups.hours.content.parse().unwrap(),
            groups.mins.map(|m| m.content).unwrap_or("0").parse().unwrap(),
            0
        ).ok_or("invalid time provided")?;

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

regex! { pub TimeRangePattern = r"^(?<hours>\d{1,2})(:(?<mins>\d{2}))?(?<op>[+-])(?<dur>\d{1,2}(\.\d)?)$" }

static TIME_RANGE_PATTERN: LazyLock<TimeRangePattern> = LazyLock::new(TimeRangePattern::new);
use std::sync::LazyLock;

use ctreg::regex;

regex! { pub DatePattern = r"^(?<year>\d{4})-(?<month>\d{1,2})-(?<day>\d{1,2})$" }

pub static DATE_PATTERN: LazyLock<DatePattern> = LazyLock::new(DatePattern::new);

regex! { pub TimeRangePattern = r"^(?<hours>\d{1,2})(:(?<mins>\d{2}))?(?<op>[+-])(?<dur>\d{1,2}(\.\d)?)$" }

pub static TIME_RANGE_PATTERN: LazyLock<TimeRangePattern> = LazyLock::new(TimeRangePattern::new);

regex! { pub ActivityPattern = r"^(?<id>\d+)$" }

pub static ACTIVITY_PATTERN: LazyLock<ActivityPattern> = LazyLock::new(ActivityPattern::new);

regex! { pub TicketPattern = r"(?<proj_key>\w+)-(?<tick_num>\d+)" }

pub static TICKET_PATTERN: LazyLock<TicketPattern> = LazyLock::new(TicketPattern::new);
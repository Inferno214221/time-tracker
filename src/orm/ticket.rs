use std::{str::FromStr, sync::LazyLock};

use ctreg::regex;
use serde::{Serialize, Serializer};

use crate::orm::model::{Ticket, TicketTime};

// So diesel doesn't support composite foreign keys but luckily, the ticket table only has two
// fields (at the moment), so we don't need to do any queries to turn a TicketTime into a Ticket.
impl From<TicketTime> for Ticket {
    fn from(value: TicketTime) -> Self {
        Ticket {
            proj_key: value.proj_key,
            tick_num: value.tick_num
        }
    }
}

impl Serialize for Ticket {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl FromStr for Ticket {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let groups = TICKET_PATTERN.captures(s).ok_or("string doesn't match ticket format")?;

        Ok(Ticket {
            proj_key: groups.proj_key.content.to_owned(),
            tick_num: groups.tick_num.content.parse().unwrap(),
        })
    }
}

regex! { pub TicketPattern = r"(?<proj_key>\w+)-(?<tick_num>\d+)" }

static TICKET_PATTERN: LazyLock<TicketPattern> = LazyLock::new(TicketPattern::new);
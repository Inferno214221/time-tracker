use std::str::FromStr;

use derive_more::{Debug, Display};
use serde::{Serialize, Serializer};

use crate::{cli::patterns::{TICKET_PATTERN, TicketPatternCaptures}, orm::model::TicketTime};

#[derive(Debug, Display, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[debug("\"{proj_key}-{tick_num}\"")]
#[display("{proj_key}-{tick_num}")]
pub struct Ticket {
    pub proj_key: String,
    pub tick_num: i32,
}

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

impl Ticket {
    pub fn try_from_match(groups: TicketPatternCaptures<'_>) -> Result<Ticket, String> {
        Ok(Ticket {
            proj_key: groups.proj_key.content.to_owned(),
            tick_num: groups.tick_num.content.parse()
                .map_err(|e| format!("Error parsing ticket number:\n{e}"))?,
        })
    }
}

impl FromStr for Ticket {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let groups = TICKET_PATTERN.captures(s)
            .ok_or("String doesn't match ticket format")?;

        Ticket::try_from_match(groups)
    }
}
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::model::{Metadata, MetadataType};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RoleConnectionData {
    pub scratcher: bool,
    pub followers: i64,
    #[serde(with = "time::serde::iso8601")]
    pub joined: OffsetDateTime,
}

pub fn metadata() -> Vec<Metadata> {
    vec![
        Metadata {
            kind: MetadataType::BooleanEqual,
            key: "scratcher".into(),
            name: "Scratcher".into(),
            name_localizations: HashMap::new(),
            description: "At least one of the user's accounts has the Scratcher status".into(),
            description_localizations: HashMap::from([(
                "pl".into(),
                "Co najmniej jedno konto użytkownika ma status Scratchera".into(),
            )]),
        },
        Metadata {
            kind: MetadataType::IntegerGreaterThanOrEqual,
            key: "followers".into(),
            name: "Followers".into(),
            name_localizations: HashMap::from([("pl".into(), "Śledzący".into())]),
            description: "The highest number of followers among the user's accounts".into(),
            description_localizations: HashMap::from([(
                "pl".into(),
                "Największa liczba śledzących ze wszystkich kont użytkownika".into(),
            )]),
        },
        Metadata {
            kind: MetadataType::DatetimeGreaterThanOrEqual,
            key: "joined".into(),
            name: "Joined".into(),
            name_localizations: HashMap::from([("pl".into(), "Data dołączenia".into())]),
            description: "Creation date of the user's oldest account".into(),
            description_localizations: HashMap::from([(
                "pl".into(),
                "Data założenia najstarszego konta użytkownika".into(),
            )]),
        },
    ]
}

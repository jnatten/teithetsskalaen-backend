use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
pub struct Teithet {
    id: i64,
    title: String,
    description: String,
    #[serde_as(as = "Rfc3339")]
    created_at: OffsetDateTime,
}

impl Teithet {
    pub fn from(teithet: crate::repository::Teithet) -> Self {
        let created_at: OffsetDateTime = teithet.created_at.assume_utc();
        Self {
            id: teithet.id,
            title: teithet.title,
            description: teithet.description,
            created_at,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateTeithet {
    pub title: String,
    pub description: String,
}

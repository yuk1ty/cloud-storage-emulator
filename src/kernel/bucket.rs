use chrono::NaiveDateTime;

pub struct Bucket {
    pub name: String,
    pub versioning: bool,
    pub time_created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

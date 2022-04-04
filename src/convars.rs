use chrono::{Utc, DateTime};
use crate::schema::{convar_ints, convar_strings};

#[derive(Queryable)]
pub struct ConvarInt {
    id: i32,
    pub key: String,
    pub value: i32,
    created_at: DateTime<Utc>
}

#[derive(Insertable)]
#[table_name = "convar_ints"]
pub struct NewConvarInt<'a> {
    pub key: &'a str,
    pub value: i32,
}

#[derive(Queryable)]
pub struct ConvarString {
    id: i32,
    pub key: String,
    pub value: String,
    created_at: DateTime<Utc>
}

#[derive(Insertable)]
#[table_name = "convar_strings"]
pub struct NewConvarString<'a> {
    pub key: &'a str,
    pub value: &'a str,
}
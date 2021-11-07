use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Technology {
    pub id: i32,
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize, Debug)]
#[table_name = "technologies"]
pub struct NewTechnology<'a> {
    pub description: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

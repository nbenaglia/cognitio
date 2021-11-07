use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use super::schema::technologies;
use super::schema::technologies::dsl::technologies as all_technologies;

#[derive(Serialize, Queryable)] 
pub struct Technology {
    pub id: i32,
    pub description: String
}
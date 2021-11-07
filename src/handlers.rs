use super::models::{NewTechnology, Technology};
use super::schema::technologies::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputTechnology {
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
}

pub async fn get_technologies(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_technologies(db))
        .await
        .map(|technology| HttpResponse::Ok().json(technology))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_all_technologies(
    pool: web::Data<Pool>,
) -> Result<Vec<Technology>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = technologies.load::<Technology>(&conn)?;
    Ok(items)
}

pub async fn get_technology_by_id(
    db: web::Data<Pool>,
    technology_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_technology_by_id(db, technology_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

// Handler for POST /technologies
pub async fn add_technology(
    db: web::Data<Pool>,
    item: web::Json<InputTechnology>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_single_technology(db, item))
        .await
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

// Handler for DELETE /technologies/{id}
pub async fn delete_technology(
    db: web::Data<Pool>,
    technology_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || delete_single_technology(db, technology_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn db_get_technology_by_id(
    pool: web::Data<Pool>,
    technology_id: i32,
) -> Result<Technology, diesel::result::Error> {
    let conn = pool.get().unwrap();
    technologies
        .find(technology_id)
        .get_result::<Technology>(&conn)
}

fn add_single_technology(
    db: web::Data<Pool>,
    item: web::Json<InputTechnology>,
) -> Result<Technology, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_technology = NewTechnology {
        description: &item.description,
        created_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(technologies)
        .values(&new_technology)
        .get_result(&conn)?;
    Ok(res)
}

fn delete_single_technology(
    db: web::Data<Pool>,
    technology_id: i32,
) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(technologies.find(technology_id)).execute(&conn)?;
    Ok(count)
}

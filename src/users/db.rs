use actix_web_dev::error::{
    Result,
    ErrorType,
    ApiError,
};
use chrono::NaiveDateTime;
use reqwest::header::PUBLIC_KEY_PINS_REPORT_ONLY;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::schema::{
    child,
    teacher,
    childandclass,
};
use ring::digest::{Context, Digest, SHA256};
use data_encoding::BASE64;

#[derive(Serialize,Deserialize,Clone,Queryable)]
pub struct Child {
    pub id: i64,
    pub name: String,
    pub last_name: String,
    pub very_last_name: String,
    pub login: String,
    pub pass_hash: String,
    pub register_data: chrono::NaiveDateTime,
}

#[derive(Serialize,Deserialize,Clone)]
pub struct NewChild {
    pub name: String,
    pub last_name: String,
    pub very_last_name: String,
    pub login: String,
    pub pass: String,
}


#[derive(Serialize,Deserialize,Clone)]
pub struct AuthData {
    pub login: String,
    pub pass: String,
}

fn make_hash(password: &str) -> String {
    let mut context = Context::new(&SHA256); 
    context.update(password.as_bytes());
    let pass_hash = context.finish();
    BASE64.encode(pass_hash.as_ref())
}

impl Child {
    pub async fn new(
        instance: &NewChild, 
        class_id: i64,
        id: Option<i64>,
        conn: &PgConnection,
    ) -> Result<()> {
        let values = if let Some(id) = id {
        diesel::insert_into(child::table)
            .values(&(
                    child::id.eq(id),
                    child::name.eq(&instance.name),
                    child::last_name.eq(&instance.last_name),
                    child::very_last_name.eq(&instance.very_last_name),
                    child::login.eq(&instance.login),
                    child::pass_hash.eq(make_hash(&instance.pass)),
            ))
            .execute(conn)?;
        diesel::insert_into(childandclass::table)
            .values(&(
                    childandclass::class_id.eq(class_id),
                    childandclass::chid_id.eq(id)
            ))
            .execute(conn)?;
        } else {
        diesel::insert_into(child::table)
            .values(&(
                    child::name.eq(&instance.name),
                    child::last_name.eq(&instance.last_name),
                    child::very_last_name.eq(&instance.very_last_name),
                    child::login.eq(&instance.login),
                    child::pass_hash.eq(make_hash(&instance.pass)),
            ))
            .execute(conn)?;
        };
        Ok(())
    }

    pub async fn from_id(
        id: i64,
        conn: &PgConnection,
    ) -> Result<Self> {
        let r = child::table
            .filter(child::id.eq(id))
            .get_results::<Self>(conn)?;
        if let Some(u) = r.get(0) {
            Ok(u.clone())
        } else {
            Err(ApiError{
                code: 404,
                message: "child not found".to_string(),
                error_type: ErrorType::Auth,
            })
        }
    }

    pub async fn get(
        creds: &AuthData,
        conn: &PgConnection,
    ) -> Result<Self> {
        let pass_hash = make_hash(&creds.pass);
        println!("hash: {}",pass_hash);
        let r = child::table
            .filter(child::login.eq(&creds.login))
            .filter(child::pass_hash.eq(pass_hash))
            .get_results::<Self>(conn)?;
        if let Some(u) = r.get(0) {
            Ok(u.clone())
        } else {
            Err(ApiError{
                code: 404,
                message: "child not found".to_string(),
                error_type: ErrorType::Auth,
            })
        }
    }
}

#[derive(Serialize,Deserialize,Clone,Queryable)]
pub struct Teacher {
    pub id: i64,
    pub name: String,
    pub last_name: String,
    pub very_last_name: String,
    pub login: String,
    pub pass_hash: String,
    pub register_data: chrono::NaiveDateTime,
}

#[derive(Serialize,Deserialize,Clone)]
pub struct NewTeacher {
    pub name: String,
    pub last_name: String,
    pub very_last_name: String,
    pub login: String,
    pub pass: String,
}

impl Teacher {
    pub async fn new(
        instance: &NewTeacher, 
        id: Option<i64>,
        conn: &PgConnection,
    ) -> Result<()> {
        let values = if let Some(id) = id {
        diesel::insert_into(teacher::table)
            .values(&(
                    teacher::id.eq(id),
                    teacher::name.eq(&instance.name),
                    teacher::last_name.eq(&instance.last_name),
                    teacher::very_last_name.eq(&instance.very_last_name),
                    teacher::login.eq(&instance.login),
                    teacher::pass_hash.eq(make_hash(&instance.pass)),
            ))
            .execute(conn)?;
        } else {
        diesel::insert_into(teacher::table)
            .values(&(
                    teacher::name.eq(&instance.name),
                    teacher::last_name.eq(&instance.last_name),
                    teacher::very_last_name.eq(&instance.very_last_name),
                    teacher::login.eq(&instance.login),
                    teacher::pass_hash.eq(make_hash(&instance.pass)),
            ))
            .execute(conn)?;
        };
        Ok(())
    }

    pub async fn from_id(
        id: i64,
        conn: &PgConnection,
    ) -> Result<Self> {
        let r = teacher::table
            .filter(teacher::id.eq(id))
            .get_results::<Self>(conn)?;
        if let Some(u) = r.get(0) {
            Ok(u.clone())
        } else {
            Err(ApiError{
                code: 404,
                message: "teacher not found".to_string(),
                error_type: ErrorType::Auth,
            })
        }
    }

    pub async fn get(
        creds: &AuthData,
        conn: &PgConnection,
    ) -> Result<Self> {
        let pass_hash = make_hash(&creds.pass);
        let r = teacher::table
            .filter(teacher::login.eq(&creds.login))
            .filter(teacher::pass_hash.eq(pass_hash))
            .get_results::<Self>(conn)?;
        if let Some(u) = r.get(0) {
            Ok(u.clone())
        } else {
            Err(ApiError{
                code: 404,
                message: "teacher not found".to_string(),
                error_type: ErrorType::Auth,
            })
        }
    }
}

use actix_web_dev::error::{
    Result,
    ErrorType,
    ApiError,
};
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::schema::{
    takenbooks,
    books,
    classes,
    booksasinged,
};
use serde_json::Value;
use diesel::sql_types::{
    Varchar,
    Timestamp,
    BigSerial,
    Bool,
};

#[derive(Serialize,Deserialize,Clone,QueryableByName,Debug)]
pub struct ChildBooks {
    #[sql_type="BigSerial"]
    pub book_id: i64,
    #[sql_type="Varchar"]
    pub book_title: String,
    #[sql_type="BigSerial"]
    pub class_id: i64,
    #[sql_type="Varchar"]
    pub class_name: String,
    #[sql_type="Bool"]
    pub is_taken: bool,
}

#[derive(Serialize,Deserialize,Clone,QueryableByName,Debug)]
pub struct TeacherBooks {
    #[sql_type="BigSerial"]
    pub class_id: i64,
    #[sql_type="Varchar"]
    pub class_name: String,
    #[sql_type="BigSerial"]
    pub book_id: i64,
    #[sql_type="Varchar"]
    pub book_title: String,
    #[sql_type="Bool"]
    pub is_asigned: bool,
}

#[derive(Serialize,Deserialize,Clone,Queryable,Insertable)]
#[table_name="takenbooks"]
pub struct TakenBooks {
    pub book_id: i64,
    pub child_id: i64,
}

#[derive(Serialize,Deserialize,Clone,Queryable,Insertable)]
#[table_name="booksasinged"]
pub struct BooksAsign {
    pub book_id: i64,
    pub class_id: i64,
}

#[derive(Serialize,Deserialize,Clone,Queryable,Insertable)]
#[table_name="books"]
pub struct Book {
    title: String,
}

#[derive(Serialize,Deserialize,Clone,Queryable)]
pub struct Books {
    id: i64,
    title: String,
}

#[derive(Serialize,Deserialize,Clone,Queryable)]
pub struct Classes {
    id: i64,
    name: String,
}

impl Classes {
    pub async fn get(
        conn: &PgConnection,
    ) -> Result<Vec<Self>> {
        let r = classes::table
            .load(conn)?;
        Ok(r)
    }
}


impl Book {
    pub async fn new(
        &self,
        conn: &PgConnection,
    ) -> Result<()> {
        diesel::insert_into(books::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }
    pub async fn delete(
        id: i64,
        conn: &PgConnection,
    ) -> Result<()> {
        diesel::delete(books::table)
            .filter(books::id.eq(id))
            .execute(conn)?;
        Ok(())
    }

    pub async fn get(
        conn: &PgConnection,
    ) -> Result<Vec<Books>> {
        let r = books::table
            .load(conn)?;
        Ok(r)
    }
}

use crate::schema::yearending;
#[derive(Serialize,Deserialize,Clone,Queryable)]
pub struct YearEnding {
    pub id: i64,
    pub is_ending: bool,
}

impl YearEnding {
    pub async fn toggle(
        conn: &PgConnection,
    ) -> Result<()> {
        diesel::update(yearending::table)
            .filter(yearending::id.eq(1))
            .set(yearending::is_ending.eq(diesel::dsl::not(yearending::is_ending)))
            .execute(conn)?;
        Ok(())
    }
    pub async fn get(
        conn: &PgConnection,
    ) -> Result<(Self)> {
        let r = yearending::table
            .filter(yearending::id.eq(1))
            .get_result::<Self>(conn)?;
        Ok(r)
    }
}



impl ChildBooks {
    pub async fn update(
        child_id: i64,
        instances: Vec<TakenBooks>, 
        conn: &PgConnection,
    ) -> Result<()> {
        diesel::delete(takenbooks::table)
            .filter(takenbooks::child_id.eq(child_id))
            .execute(conn)?;
        diesel::insert_into(takenbooks::table)
            .values(instances)
            .execute(conn)?;
        Ok(())
    }

    pub async fn get(
        child_id: i64,
        conn: &PgConnection,
    ) -> Result<Vec<ChildBooks>> {
        let r = diesel::sql_query("SELECT * FROM ChildBookView where child_id=$1;")
            .bind::<BigSerial,_>(child_id)
            .get_results(conn)?;
        println!("view: {:?}",r);
        Ok(r)
    }
}

impl BooksAsign {
    pub async fn update(
        instances: Vec<BooksAsign>, 
        conn: &PgConnection,
    ) -> Result<()> {
        diesel::delete(booksasinged::table)
            .execute(conn)?;
        diesel::insert_into(booksasinged::table)
            .values(instances)
            .execute(conn)?;
        Ok(())
    }

    pub async fn get(
        conn: &PgConnection,
    ) -> Result<Vec<TeacherBooks>> {
        let r = diesel::sql_query("select * from AssignedBooks;")
            .get_results(conn)?;
        println!("view: {:?}",r);
        Ok(r)
    }
}

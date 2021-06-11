use actix_web::{
    web, http, dev, guard,
    App, HttpResponse, client::Client,
    HttpServer, HttpRequest, Responder,
};
use serde::Deserialize;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use reqwest;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use actix_web_dev::error::{
    Result,
    ApiError,
    ErrorType,
};
use actix_web_dev::auth::{
    Auth,
};
use super::db::{
    YearEnding,
    ChildBooks,
    BooksAsign,
    Classes,
    TakenBooks,
    Book,
};

pub fn lib_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/library")
        .route("/year_is_ended", web::post().to(get_year_ending))
        .route("/toggle_year", web::post().to(toggle_year_ending))
        .route("/get_child_books", web::post().to(get_child_books))
        .route("/update_child_books", web::post().to(update_child_books))
        .route("/get_teacher_books", web::post().to(get_teacher_books))
        .route("/update_teacher_books", web::post().to(update_teacher_books))
        .route("/add_book", web::post().to(add_book))
        .route("/delete_book", web::post().to(delete_book))
        .route("/get_books", web::post().to(get_books))
        .route("/get_classes", web::post().to(get_classes))
    );
}

pub async fn get_year_ending(
    conn: web::Data<DbPool>,
    _req: HttpRequest
) -> Result<HttpResponse> {
    let conn = conn.get()?;
    let r = YearEnding::get(&conn).await?;
    Ok(HttpResponse::Ok().json(r))
}


pub async fn toggle_year_ending(
    auth: Auth,
    conn: web::Data<DbPool>,
    _req: HttpRequest
) -> Result<HttpResponse> {
    require!(auth.roles.contains(&"admin".to_string()),"not a admin");
    let conn = conn.get()?;
    YearEnding::toggle(&conn).await?;
    Ok(HttpResponse::Ok().json(""))
}

pub async fn update_child_books(
    auth: Auth,
    form: web::Json<Vec<TakenBooks>>,
    conn: web::Data<DbPool>,
    req: HttpRequest
) -> Result<HttpResponse> {
    require!(auth.roles.contains(&"child".to_string()),"not admin");
    let conn = conn.get()?;
    ChildBooks::update(auth.id, form.into_inner(), &conn).await?;
    Ok(HttpResponse::Ok().json(""))
}

pub async fn get_child_books(
    auth: Auth,
    conn: web::Data<DbPool>,
    req: HttpRequest
) -> Result<HttpResponse> {
    require!(auth.roles.contains(&"child".to_string()),"not admin");
    let conn = conn.get()?;
    let r = ChildBooks::get(auth.id, &conn).await?;
    Ok(HttpResponse::Ok().json(r))
}

pub async fn update_teacher_books(
    auth: Auth,
    form: web::Json<Vec<BooksAsign>>,
    conn: web::Data<DbPool>,
    req: HttpRequest
) -> Result<HttpResponse> {
    require!(auth.roles.contains(&"teacher".to_string()),"not admin");
    let conn = conn.get()?;
    BooksAsign::update(form.into_inner(), &conn).await?;
    Ok(HttpResponse::Ok().json(""))
}

pub async fn get_classes(
    conn: web::Data<DbPool>,
    req: HttpRequest
) -> Result<HttpResponse> {
    let conn = conn.get()?;
    let r = Classes::get(&conn).await?;
    Ok(HttpResponse::Ok().json(r))
}

pub async fn get_books(
    conn: web::Data<DbPool>,
    req: HttpRequest
) -> Result<HttpResponse> {
    let conn = conn.get()?;
    let r = Book::get(&conn).await?;
    Ok(HttpResponse::Ok().json(r))
}

pub async fn add_book(
    auth: Auth,
    data: web::Json<Book>,
    conn: web::Data<DbPool>,
    req: HttpRequest
) -> Result<HttpResponse> {
    require!(auth.roles.contains(&"admin".to_string()),"not admin");
    let conn = conn.get()?;
    data.into_inner().new(&conn).await?;
    Ok(HttpResponse::Ok().json("{}"))
}

#[derive(Deserialize)]
pub struct Id {
    id: i64,
}

pub async fn delete_book(
    auth: Auth,
    form: web::Json<Id>,
    conn: web::Data<DbPool>,
    req: HttpRequest
) -> Result<HttpResponse> {
    require!(auth.roles.contains(&"admin".to_string()),"not admin");
    let conn = conn.get()?;
    Book::delete(form.id, &conn).await?;
    Ok(HttpResponse::Ok().json("{}"))
}

pub async fn get_teacher_books(
    auth: Auth,
    conn: web::Data<DbPool>,
    req: HttpRequest
) -> Result<HttpResponse> {
    require!(auth.roles.contains(&"teacher".to_string()),"not admin");
    let conn = conn.get()?;
    let r = BooksAsign::get(&conn).await?;
    Ok(HttpResponse::Ok().json(r))
}

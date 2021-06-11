use actix_web::{
    web, http, dev, guard,
    App, HttpResponse, client::Client,
    HttpServer, HttpRequest, Responder,
};
use serde::Deserialize;
use futures_util::TryFutureExt;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use actix_web_dev::error::{
    Result,
    ApiError,
    ErrorType,
};
use actix_web_dev::auth::{
    Auth,
    AuthSecret,
};
use super::db::{
    AuthData,
    Teacher,
    NewTeacher,
    NewChild,
    Child,
};

pub fn users_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users")
        .route("/create_teacher", web::post().to(create_teacher))
        .route("/create_child", web::post().to(create_child))
        .route("/login", web::post().to(login))
    );
}

pub async fn create_teacher(
    auth: Auth,
    form: web::Json<NewTeacher>,
    conn: web::Data<DbPool>,
) -> Result<HttpResponse> {
    require!(auth.roles.contains(&"admin".to_string()),"not a admin");
    let conn = conn.get()?;
    let form = form.into_inner();
    let roles = &vec!["teacher".to_string()];
    let a = Auth::new(&form.login, "plain", roles, &conn).await?;
    let s = Teacher::new(&form,Some(a.id), &conn).await?;
    Ok(HttpResponse::Ok().json(""))
}

#[derive(Deserialize)]
pub struct NCh {
    pub instance: NewChild,
    pub class_id: i64,
}

pub async fn create_child(
    auth: Auth,
    form: web::Json<NCh>,
    conn: web::Data<DbPool>,
) -> Result<HttpResponse> {
    require!(auth.roles.contains(&"admin".to_string()),"not a admin");
    let conn = conn.get()?;
    let form = form.into_inner();
    let roles = &vec!["child".to_string()];
    let a = Auth::new(&form.instance.login, "plain", roles, &conn).await?;
    let s = Child::new(&form.instance,form.class_id,Some(a.id), &conn).await?;
    Ok(HttpResponse::Ok().json(""))
}

pub async fn login(
    form: web::Json<AuthData>,
    conn: web::Data<DbPool>,
    secret: web::Data<AuthSecret>,
    req: HttpRequest
) -> Result<HttpResponse> {
    let conn = conn.get()?;
    let form = form.into_inner();
    if let Ok(teacher) = Teacher::get(&form, &conn).await {
        let auth = Auth::get(&form.login, "plain", &conn).await?;
        let jwt = auth.get_jwt(&secret).await?;
        return Ok(HttpResponse::Ok().json(json!({
            "jwt":jwt,
            "teacher":teacher,
            "is_admin":auth.roles.contains(&"admin".to_string()),
        })));
    }
    if let Ok(child) = Child::get(&form, &conn).await {
        let auth = Auth::get(&form.login, "plain", &conn).await?;
        let jwt = auth.get_jwt(&secret).await?;
        return Ok(HttpResponse::Ok().json(json!({
            "jwt":jwt,
            "seller":child,
            "is_admin":auth.roles.contains(&"admin".to_string()),
        })));
    }
    Err(ApiError{
        code: 402,
        message: "invalid login or password or user not exists".to_string(),
        error_type: ErrorType::Auth,
    })
}

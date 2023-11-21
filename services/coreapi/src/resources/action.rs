use actix_web::{
    delete, get, http::header::ContentType, 
    patch, post, web, HttpResponse, Responder, Result, Error as WebError,
    Scope, error::ErrorBadRequest,
};
use serde::Deserialize;
use synlib::models::Action;
use uuid::Uuid;

#[get("/{id}")]
async fn _read(path: web::Path<String>) -> Result<impl Responder> {
    let action_id = Uuid::parse_str(&path.into_inner())
        .map_err(|e| ErrorBadRequest(e))?;

    Ok(web::Json(Action {
        id: Some(action_id), // Ensure you have an 'id' field in your Action struct
        description: String::from("Example Action"),
        entry_point: String::from("example_action.py"),
        name: String::from("example_pack.example_action"),
        runner_type: String::from("python"),
        enabled: true,
        parameters: None, // Ensure this matches your Action struct definition
    }))
}

#[derive(Deserialize)]
struct ActionFilter {
    pub pack: Option<String>,
    pub status: Option<String>,
    #[serde(rename = "ref")]
    pub _ref: Option<String>
}

#[get("")]
// async fn _list(filter: web::Query<ActionFilter>)  -> Result<impl Responder, Error> {
async fn _list() -> Result<impl Responder> {
    let actions: Vec<Action> = vec![Action {
        id: Some(Uuid::new_v4()),
        description: String::from("Example Action"),
        entry_point: String::from("example_action.py"),
        name: String::from("example_pack.example_action"),
        runner_type: String::from("python"),
        enabled: true,
        parameters: None,
    }];
    Ok(web::Json(actions))
}

#[post("/")]
async fn _create(action: web::Json<Action>) -> HttpResponse {
    HttpResponse::Created()
        .content_type(ContentType::plaintext())
        .body(Uuid::new_v4().to_string())
}

#[patch("/{id}")]
async fn _update() -> HttpResponse {
    HttpResponse::Ok().into()
}

#[delete("/{id}")]
async fn _delete() -> HttpResponse {
    HttpResponse::NoContent().into()
}

pub fn build_scope() -> Scope {
    web::scope("/action")
        .service(_read)
        .service(_list)
        .service(_create)
        .service(_update)
        .service(_delete)
}

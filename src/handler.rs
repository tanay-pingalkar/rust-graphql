use crate::resolver::root::*;
use crate::AppState;
use actix_web::{get, post, web, HttpResponse, Responder};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

#[post("/graphql")]
pub async fn graphql(
    app_state: web::Data<AppState>,
    data: web::Json<GraphQLRequest>,
) -> impl Responder {
    let ctx = Context {
        pool: app_state.pool.clone(),
    };
    let user = web::block(move || {
        let res = data.execute_sync(&app_state.schema, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await
    .unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(user)
}

#[get("/graphql")]
pub async fn graphiql() -> impl Responder {
    let html = graphiql_source("http://localhost:5000/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

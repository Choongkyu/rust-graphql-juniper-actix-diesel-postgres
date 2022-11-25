use super::context::GraphQLContext;
use super::db::PostgresPool;
use super::graphql::create_schema;
use super::graphql::Schema;
use actix_web::Responder;
use actix_web::{web, Error, HttpResponse};
use actix_web_lab::respond::Html;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std::sync::Arc;

// The configuration callback that enables us to add the /graphql route
// to the actix-web server.
pub fn graphql_endpoints(config: &mut web::ServiceConfig) {
    let schema = Arc::new(create_schema());
    config
        .app_data(web::Data::new(schema))
        .route("/graphql", web::post().to(graphql_route))
        .route("/graphql", web::get().to(graphql_graphiql));
}

// The GraphQL Playground route.
async fn graphql_graphiql() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

// The core handler that provides all GraphQL functionality.
async fn graphql_route(
    // The DB connection pool
    pool: web::Data<PostgresPool>,
    // The GraphQL schema
    schema: web::Data<Arc<Schema>>,
    // The incoming HTTP request
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    // Instantiate a context
    let ctx = GraphQLContext {
        pool: pool.get_ref().to_owned(),
    };

    // Handle the incoming request and return a string result (or error)
    let res = data.execute(&schema, &ctx).await;

    // Return the string as a JSON payload
    Ok(HttpResponse::Ok().json(res))
}

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use async_std;
use futures;
use hyper::{
    server::Server,
    service::{make_service_fn, service_fn},
    Body, Request, Response,
};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
#[allow(unused)] use log::{debug, error, info, trace, warn};
use std::{convert::Infallible, io, sync::Arc};
use tokio;
#[macro_use] extern crate juniper;

mod schema {
    use juniper::{FieldResult, RootNode};

    #[derive(GraphQLEnum)]
    enum Episode {
        NewHope,
        Empire,
        Jedi,
    }

    #[derive(GraphQLObject)]
    #[graphql(description = "A humanoid creature in the Star Wars universe")]
    struct Human {
        id:          String,
        name:        String,
        appears_in:  Vec<Episode>,
        home_planet: String,
    }

    #[derive(GraphQLInputObject)]
    #[graphql(description = "A humanoid creature in the Star Wars universe")]
    struct NewHuman {
        name:        String,
        appears_in:  Vec<Episode>,
        home_planet: String,
    }

    pub struct QueryRoot;

    graphql_object!(QueryRoot: () |&self| {
    field human(&executor, id: String) -> FieldResult<Human> {
        Ok(Human{
            id: "1234".to_owned(),
            name: "Luke".to_owned(),
            appears_in: vec![Episode::NewHope],
            home_planet: "Mars".to_owned(),
        })
    }
});

    pub struct MutationRoot;

    graphql_object!(MutationRoot: () |&self| {
    field createHuman(&executor, new_human: NewHuman) -> FieldResult<Human> {
        Ok(Human{
            id: "1234".to_owned(),
            name: new_human.name,
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
        })
    }
});

    pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

    pub fn create_schema() -> Schema {
        Schema::new(QueryRoot {}, MutationRoot {})
    }
}

use crate::schema::{create_schema, Schema};

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    // Create Juniper schema
    let schema = std::sync::Arc::new(create_schema());

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

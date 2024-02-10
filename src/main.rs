use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_poem::GraphQL;
use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};

type DynError = Box<dyn std::error::Error + Send + Sync>;
type Result<T = (), E = String> = std::result::Result<T, E>;

struct Query;

#[Object]
impl Query {
    async fn foo(&self) -> Result<Option<String>> {
        Ok(Some("Hello from foo()".into()))
    }

    async fn bar(&self) -> Result<Option<String>> {
        let value = might_error().await?;
        // [some long function]
        Ok(Some(value))
    }

    // Inverting Result/Option:
    async fn inverted(&self) -> Option<Result<String>> {
        // Note that having to return Option<Result<String>> means that you can't use `?`:
        // This would not work here:
        //
        // let value = might_error().await?;
        // Some(Ok(value))

        Some(Err("Some error".into()))
    }
}

// simulate a user calling a fallible function.
async fn might_error() -> Result<String> {
    Err("Some error".into())
}

// Below, Grabbed from the async-graphql readme:

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().finish())
}

#[tokio::main]
async fn main() -> Result<(), DynError> {
    // create the schema
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    // start the http server
    let app = Route::new().at("/", get(graphiql).post(GraphQL::new(schema)));
    println!("GraphiQL: http://localhost:8000");
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await?;
    Ok(())
}

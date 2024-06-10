use axum::{response::Html, routing::get, Router};
use redis::Commands;
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler)).merge(Router::new().route("/items", get(items)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html(include_str!("../index.html"))
}

async fn items() -> Html<&'static str> {
    basics();
    Html(include_str!("../index.html"))
}

fn basics() {
    let mut conn = connect();
    println!("******* Running SET, GET, INCR commands *******");

    let _: () = redis::cmd("SET")
        .arg("foo")
        .arg("bar")
        .query(&mut conn)
        .expect("failed to execute SET for 'foo'");

    let bar: String = redis::cmd("GET")
        .arg("foo")
        .query(&mut conn)
        .expect("failed to execute GET for 'foo'");
    println!("value for 'foo' = {}", bar);
}

fn connect() -> redis::Connection {
    let redis_host_name =
        env::var("REDIS_HOSTNAME").expect("missing environment variable REDIS_HOSTNAME");

    let redis_password = env::var("REDIS_PASSWORD").unwrap_or_default();

    let uri_scheme = match env::var("IS_TLS") {
        Ok(_) => "rediss",
        Err(_) => "redis",
    };

    let redis_conn_url = format!("{}://:{}@{}", uri_scheme, redis_password, redis_host_name);

    redis::Client::open(redis_conn_url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("Failed to connect to Redis")
}

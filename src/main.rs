use warp::Filter;

mod filters;
mod handlers;
mod models;

pub fn config_env_var(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|e| format!("{}: {}", name, e))
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // todo firestore
    let db = models::blank_db();
    let api = filters::articles(db);

    let routes = api.with(warp::log("articles"));

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}

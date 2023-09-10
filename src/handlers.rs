use std::convert::Infallible;

use warp::Reply;

use super::models::{Article, Db};

pub async fn list_articles(db: Db) -> Result<impl Reply, Infallible> {
    let articles = db.lock().await;

    // chain
    let articles: Vec<Article> = articles.clone().into_iter().collect();
    Ok(warp::reply::json(&articles))
}

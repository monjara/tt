use warp::{Filter, Rejection, Reply};

use crate::handlers;

use super::models::Db;

pub fn articles(db: Db) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    article_list(db)
}

pub fn article_list(db: Db) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("articles")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::list_articles)
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

// TODO: firestore
pub type Db = Arc<Mutex<Vec<Article>>>;

pub fn blank_db() -> Db {
    Arc::new(Mutex::new(Vec::new()))
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Article {
    pub article_id: u64,
    pub title: String,
    pub content: String,
    pub categories: Vec<Category>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Category {
    pub category_id: u64,
    pub category_name: String,
}

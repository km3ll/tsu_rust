#![allow(unused)]
// Import serialize and deserialize from serde
use serde::{Serialize, Deserialize};

// Derive traits for Post struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    title: String,
    body: String,
}

impl Post {
    pub fn new(title: String, body: String) -> Post {
        Post { title, body }
    }
}
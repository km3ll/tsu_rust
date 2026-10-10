use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Zettel {
	pub id: i32,
	pub content: String,
}

impl Zettel {
	pub fn new(id: i32, content: String) -> Self {
		Self { id, content }
	}
}

#[derive(Deserialize, Serialize)]
pub struct FindAllZettelsResponse {
	pub zettels: Vec<Zettel>,
}

impl FindAllZettelsResponse {
	pub fn new(zettels: Vec<Zettel>) -> Self {
		Self { zettels }
	}
}

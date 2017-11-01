#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

mod schema;
mod models;

fn main() {
    use schema::recipes::dsl::*;

    let connection = establish_connection();
    let results = recipes
        .limit(5)
        .load::<models::Recipe>(&connection)
        .expect("Error loading recipes");

    println!("Displaying {} recipes", results.len());
    for recipe in results {
        println!("{}", recipe.name);
    }
}

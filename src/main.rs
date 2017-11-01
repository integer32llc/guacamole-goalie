extern crate clap;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;

use clap::{App, Arg, SubCommand};
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

    let list_recipes_cmd = SubCommand::with_name("list-recipes")
        .about("Lists recipes in the database");

    let add_recipe_cmd = SubCommand::with_name("add-recipe")
        .about("Add a recipe with the specified name to the database")
        .arg(Arg::with_name("name")
             .long("name")
             .takes_value(true)
             .help("the name of the recipe to create")
             .required(true));

    let matches = App::new("guacamole-goalie")
        .subcommand(list_recipes_cmd)
        .subcommand(add_recipe_cmd)
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("list-recipes") {
        list_recipes(matches)
    }

    if let Some(matches) = matches.subcommand_matches("add-recipe") {
        add_recipe(matches)
    }
}

fn list_recipes(_matches: &clap::ArgMatches) {
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

fn add_recipe(matches: &clap::ArgMatches) {
    let name_arg_value = matches.value_of("name")
        .expect("Recipe name required");

    use schema::recipes::dsl::*;

    let connection = establish_connection();

    diesel::insert_into(recipes)
        .values(&name.eq(name_arg_value))
        .execute(&connection)
        .expect("Could not insert recipe");
}

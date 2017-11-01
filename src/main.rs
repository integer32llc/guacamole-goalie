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

use models::Recipe;

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

    let add_ingredient_cmd = SubCommand::with_name("add-ingredient")
        .about("Add an ingredient to a recipe")
        .arg(Arg::with_name("recipe")
             .long("recipe")
             .takes_value(true)
             .help("the name of the recipe to add an ingredient to")
             .required(true))
        .arg(Arg::with_name("amount")
             .long("amount")
             .takes_value(true)
             .help("the amount of the ingredient in the recipe")
             .required(true))
        .arg(Arg::with_name("name")
             .long("name")
             .takes_value(true)
             .help("the name of the ingredient to add")
             .required(true));

    let matches = App::new("guacamole-goalie")
        .subcommand(list_recipes_cmd)
        .subcommand(add_recipe_cmd)
        .subcommand(add_ingredient_cmd)
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("list-recipes") {
        list_recipes(matches)
    }

    if let Some(matches) = matches.subcommand_matches("add-recipe") {
        add_recipe(matches)
    }

    if let Some(matches) = matches.subcommand_matches("add-ingredient") {
        add_ingredient(matches)
    }
}

fn list_recipes(_matches: &clap::ArgMatches) {
    use schema::recipes::dsl::*;

    let connection = establish_connection();
    let results = recipes
        .limit(5)
        .load::<Recipe>(&connection)
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

fn add_ingredient(matches: &clap::ArgMatches) {
    let recipe_arg_value = matches.value_of("recipe")
        .expect("Recipe name required");

    let ingredient_amount_value = matches.value_of("amount")
        .expect("Ingredient amount required");

    let ingredient_name_value = matches.value_of("name")
        .expect("Ingredient name required");

    use schema::{recipes, ingredients};

    let connection = establish_connection();

    let recipe = recipes::table
        .filter(recipes::name.eq(recipe_arg_value))
        .first::<Recipe>(&connection)
        .expect("Could not find recipe");

    diesel::insert_into(ingredients::table)
        .values(&(
            ingredients::name.eq(ingredient_name_value),
            ingredients::amount.eq(ingredient_amount_value),
            ingredients::recipe_id.eq(recipe.id)
        ))
        .execute(&connection)
        .expect("Could not insert ingredient");

    println!("Added {} {} to recipe {}", ingredient_amount_value, ingredient_name_value, recipe_arg_value);
}

use schema::{recipes, ingredients};

#[derive(Queryable, Identifiable)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Associations, Identifiable)]
#[belongs_to(Recipe)]
pub struct Ingredient {
    pub id: i32,
    pub amount: String,
    pub name: String,
    pub recipe_id: i32,
}

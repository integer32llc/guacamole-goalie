#[derive(Queryable)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable)]
pub struct Ingredient {
    pub id: i32,
    pub amount: String,
    pub name: String,
    pub recipe_id: i32,
}

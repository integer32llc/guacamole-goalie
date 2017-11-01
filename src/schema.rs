table! {
    ingredients (id) {
        id -> Int4,
        amount -> Varchar,
        name -> Varchar,
        recipe_id -> Int4,
    }
}

table! {
    recipes (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(ingredients -> recipes (recipe_id));

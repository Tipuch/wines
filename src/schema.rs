table! {
    use types::WineColor;
    use diesel::sql_types::*;
    saq_wines (id) {
        id -> Int4,
        name -> Varchar,
        country -> Varchar,
        region -> Varchar,
        designation_of_origin -> Varchar,
        regulated_designation -> Bool,
        producer -> Varchar,
        volume -> Numeric,
        price -> Numeric,
        alcohol_percent -> Numeric,
        color -> WineColor,
        grape_varieties -> Array<Text>,
        available_online -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        admin -> Bool,
        salt -> Bytea,
        password -> Bytea,
    }
}

table! {
    use types::WineColor;
    use diesel::sql_types::*;
    wine_recommendations (id) {
        id -> Int4,
        country -> Varchar,
        region -> Varchar,
        designation_of_origin -> Varchar,
        producer -> Varchar,
        rating -> Int4,
        color -> WineColor,
        grape_variety -> Varchar,
        user_id -> Nullable<Int4>,
        wine_name -> Varchar,
    }
}

joinable!(wine_recommendations -> users (user_id));

allow_tables_to_appear_in_same_query!(
    saq_wines,
    users,
    wine_recommendations,
);

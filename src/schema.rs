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
    }
}

allow_tables_to_appear_in_same_query!(
    saq_wines,
    wine_recommendations,
);

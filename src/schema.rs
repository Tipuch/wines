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
        alcohol_percent -> Numeric,
        color -> WineColor,
        grape_varieties -> Array<Text>,
    }
}

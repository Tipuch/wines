table! {
    saq_wine (id) {
        id -> Int4,
        name -> Varchar,
        country -> Varchar,
        region -> Varchar,
        designation_of_origin -> Varchar,
        regulated_designation -> Bool,
        producer -> Varchar,
        volume -> Numeric,
        alcohol_percent -> Int4,
        color -> Wine_color,
        grape_varieties -> Nullable<Array<Text>>,
    }
}

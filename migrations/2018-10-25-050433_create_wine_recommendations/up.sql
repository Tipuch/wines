CREATE TABLE wine_recommendations
(
    id SERIAL PRIMARY KEY,
    country varchar NOT NULL,
    region varchar NOT NULL,
    designation_of_origin varchar NOT NULL,
    producer varchar NOT NULL,
    rating integer NOT NULL,
    color wine_color NOT NULL,
    grape_variety varchar NOT NULL
);
CREATE INDEX wine_recommendations_rating ON wine_recommendations (rating);
CREATE INDEX wine_recommendations_country_region_idx ON wine_recommendations (country, region);
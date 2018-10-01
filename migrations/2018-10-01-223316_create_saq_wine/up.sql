CREATE TYPE wine_color AS ENUM ('red', 'white', 'pink');
CREATE TABLE saq_wine
(
    id SERIAL PRIMARY KEY,
    name varchar NOT NULL UNIQUE CHECK (name <> ''),
    country varchar NOT NULL,
    region varchar NOT NULL,
    designation_of_origin varchar NOT NULL,
    regulated_designation boolean NOT NULL,
    producer varchar NOT NULL,
    volume decimal NOT NULL,
    alcohol_percent integer NOT NULL,
    color wine_color NOT NULL,
    grape_varieties text[][2]
);
CREATE INDEX saq_wine_name_idx ON saq_wine (name);
CREATE INDEX saq_wine_country_region_idx ON saq_wine (country, region);

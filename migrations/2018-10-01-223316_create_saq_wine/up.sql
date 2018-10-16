CREATE TYPE wine_color AS ENUM ('red', 'white', 'pink');
CREATE TABLE saq_wines
(
    id SERIAL PRIMARY KEY,
    name varchar NOT NULL UNIQUE CHECK (name <> ''),
    country varchar NOT NULL,
    region varchar NOT NULL,
    designation_of_origin varchar NOT NULL,
    regulated_designation boolean NOT NULL,
    producer varchar NOT NULL,
    volume decimal NOT NULL,
    alcohol_percent decimal NOT NULL,
    color wine_color NOT NULL,
    grape_varieties text[] NOT NULL
);
CREATE INDEX saq_wines_name_idx ON saq_wines (name);
CREATE INDEX saq_wines_country_region_idx ON saq_wines (country, region);

DROP INDEX saq_wines_country_region_idx;
DROP INDEX saq_wines_name_idx;
CREATE INDEX saq_wines_multi_idx ON saq_wines (color, country, region, designation_of_origin, price, name, grape_varieties);

DROP INDEX wine_recommendations_country_region_idx;
DROP INDEX wine_recommendations_rating;
CREATE INDEX wine_recommendations_multi_idx ON wine_recommendations (color, country, region, designation_of_origin, grape_variety, wine_name)
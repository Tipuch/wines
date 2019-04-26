DROP INDEX saq_wines_multi_idx;
CREATE INDEX saq_wines_name_idx ON saq_wines (name);
CREATE INDEX saq_wines_country_region_idx ON saq_wines (country, region);

DROP INDEX wine_recommendations_multi_idx;
CREATE INDEX wine_recommendations_rating ON wine_recommendations (rating);
CREATE INDEX wine_recommendations_country_region_idx ON wine_recommendations (country, region);

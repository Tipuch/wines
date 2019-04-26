ALTER TABLE wine_recommendations ADD COLUMN wine_name varchar NOT NULL DEFAULT '';
ALTER TABLE wine_recommendations ALTER COLUMN wine_name DROP DEFAULT;
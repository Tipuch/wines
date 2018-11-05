ALTER TABLE saq_wines ADD COLUMN available_online BOOLEAN NOT NULL DEFAULT true;
ALTER TABLE saq_wines ALTER COLUMN available_online DROP DEFAULT;
-- Add down migration script here
DROP TABLE IF EXISTS translations;
DROP TABLE IF EXISTS translators;
DROP TABLE IF EXISTS documents;
DROP TABLE IF EXISTS users;
DROP TYPE IF EXISTS lang;
DROP INDEX IF EXISTS user_username_unique_lower_idx;
DROP INDEX IF EXISTS user_email_unique_lower_idx;
DROP INDEX IF EXISTS user_version_code_idx;

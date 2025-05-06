-- migrations/20250506155245_remove_salt_from_users.sql

ALTER TABLE users DROP COLUMN salt;
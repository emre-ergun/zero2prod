-- migrations/20250506143450_rename_password_column.sql

ALTER TABLE users RENAME password TO password_hash
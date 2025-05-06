-- migrations/20250506135651_create_users_table.sql

CREATE TABLE users(
    user_id uuid PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
)

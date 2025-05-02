-- /migrations/20250502123552_add_status_to_subscriptions.sql

ALTER TABLE subscriptions ADD COLUMN status TEXT NULL;

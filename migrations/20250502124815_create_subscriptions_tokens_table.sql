-- /migrations/20250502124815_create_subscriptions_tokens_table.sql

CREATE TABLE subscription_tokens(
    subscription_token TEXT NOT NULL,
    subscriber_id uuid NOT NULL
        REFERENCES subscriptions (id),
    PRIMARY KEY (subscription_token)
);
-- 20250515144052_create_issue_delivery_queue_table.sql

CREATE TABLE issue_delivery_queue (
    newsletter_issue_id uuid NOT NULL
    REFERENCES newsletter_issues (newsletter_issue_id),
    subscriber_email TEXT NOT NULL,
    PRIMARY KEY(newsletter_issue_id, subscriber_email)
);
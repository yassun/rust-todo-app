-- Add migration script here
CREATE TABLE subscriptions(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    content TEXT NOT NULL,
    done BOOLEAN NOT NULL,
    created_at timestamptz NOT NULL
);

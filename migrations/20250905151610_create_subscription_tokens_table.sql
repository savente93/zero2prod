create table subscription_tokens(
    subscription_token TEXT NOT NULL,
    subscriber_id uuid NOT NULL
    REFERENCES subscriptions (id),
primary key (subscription_token)
);

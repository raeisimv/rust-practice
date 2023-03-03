CREATE TABLE subscriptions
(
    id            uuid         NOT NULL,
    PRIMARY KEY (id),
    email         VARCHAR(512) NOT NULL UNIQUE,
    name          VARCHAR(128) NOT NULL,
    subscribed_at timestamptz  NOT NULL
);


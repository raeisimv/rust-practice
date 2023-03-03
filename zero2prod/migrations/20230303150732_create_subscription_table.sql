
-- Keep track of migrations
CREATE TABLE IF NOT EXISTS ___migrations
(
    id            TEXT        NOT NULL,
    PRIMARY KEY (id),
    subscribed_at timestamptz NOT NULL
);

DO
$$
    BEGIN
        IF NOT EXISTS(SELECT id FROM ___migrations WHERE id = '20230303150732_create_subscription_table') THEN
            CREATE TABLE subscriptions
            (
                id            uuid         NOT NULL,
                PRIMARY KEY (id),
                email         VARCHAR(512) NOT NULL UNIQUE,
                name          VARCHAR(128) NOT NULL,
                subscribed_at timestamptz  NOT NULL
            );
            INSERT INTO ___migrations VALUES ('20230303150732_create_subscription_table', now());
            ELSE
        END IF;
    END;
$$


-- Your SQL goes here
CREATE TABLE events (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    name STRING NOT NULL,
    description STRING NOT NULL,
    category STRING NOT NULL,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    min_amount BIGINT NOT NULL,
    max_amount BIGINT NOT NULL,
    user_id UUID NOT NULL,
    PRIMARY KEY (id),
    INDEX (category),
    FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TABLE event_members (
    event_id UUID NOT NULL,
    user_id UUID NOT NULL,
    amount BIGINT NOT NULL,
    PRIMARY KEY (event_id, user_id),
    FOREIGN KEY (event_id) REFERENCES events (id),
    FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TABLE event_comments (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    event_id UUID NOT NULL,
    user_id UUID NOT NULL,
    content STRING NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (event_id) REFERENCES events (id),
    FOREIGN KEY (user_id) REFERENCES users (id)
);
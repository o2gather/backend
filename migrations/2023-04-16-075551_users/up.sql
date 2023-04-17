-- Your SQL goes here
CREATE TABLE users (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    phone VARCHAR(13) NOT NULL default '',
    avatar STRING NOT NULL,
    guid STRING NOT NULL UNIQUE,
    PRIMARY KEY (id),
    INDEX (guid)
);
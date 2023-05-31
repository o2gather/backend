-- Your SQL goes here
ALTER TABLE event_comments ADD create_at TIMESTAMP NOT NULL
    DEFAULT CURRENT_TIMESTAMP;
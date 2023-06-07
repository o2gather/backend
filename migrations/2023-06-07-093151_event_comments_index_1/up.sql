-- Your SQL goes here
CREATE INDEX ON event_comments (event_id) STORING (user_id, content, created_at);
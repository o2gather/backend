-- This file should undo anything in `up.sql`
-- Your SQL goes here
ALTER TABLE event_comments 
drop CONSTRAINT event_comments_event_id_fkey_on_delete;

ALTER TABLE event_comments
ADD CONSTRAINT event_comments_event_id_fkey
    FOREIGN KEY (event_id)
    REFERENCES events (id);
-- Your SQL goes here
ALTER TABLE event_comments DROP CONSTRAINT event_comments_event_id_fkey;
ALTER TABLE event_comments ADD CONSTRAINT event_comments_event_id_fkey_on_delete FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE CASCADE;
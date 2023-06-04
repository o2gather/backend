-- This file should undo anything in `up.sql`
ALTER TABLE event_comments RENAME COLUMN created_at TO create_at;
-- This file should undo anything in `up.sql`
DROP INDEX ON event_comments (event_id) STORING (user_id, content, created_at);
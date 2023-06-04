-- Your SQL goes here
ALTER TABLE events ADD established BOOLEAN NOT NULL DEFAULT FALSE;
-- Then you can use the `established` column to determine whether the event is established or not.
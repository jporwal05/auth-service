-- This file should undo anything in `up.sql`
ALTER TABLE users
DROP COLUMN password,
DROP COLUMN created_at,
DROP COLUMN modified_at;
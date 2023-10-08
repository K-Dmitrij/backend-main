-- This file should undo anything in `up.sql`
ALTER TABLE message ALTER COLUMN created_at TYPE TIMESTAMP;

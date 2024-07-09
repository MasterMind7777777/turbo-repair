-- down.sql

DROP TABLE IF EXISTS task_staff_links;
DROP TABLE IF EXISTS staff_tasks;
DROP TABLE IF EXISTS user_requests;

-- Optionally drop the extension if no longer needed
DROP EXTENSION IF EXISTS "uuid-ossp";

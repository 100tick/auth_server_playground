-- Add down migration script here
CREATE TABLE IF EXISTS "todo" (
    "id" BIGSERIAL PRIMARY KEY,
    "user_id" BIGINT NOT NULL UNIQUE,
    "title" VARCHAR(50) NOT NULL,
    -- pending, progress, done
    "begin_at" "end_at" "state" VARCHAR NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT "user_to_many_todos" FOREIGN KEY ("user_id") REFERENCES "user"("id") ON UPDATE CASCADE ON DELETE CASCADE
);
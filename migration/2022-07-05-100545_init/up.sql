-- CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE OR REPLACE FUNCTION public.update_updated_at() RETURNS TRIGGER AS $$ BEGIN NEW.updated_at = NOW();

RETURN NEW;

END;

$$ language 'plpgsql';

CREATE TABLE IF NOT EXISTS "user" (
    -- "id" UUID PRIMARY KEY NOT NULL DEFAULT UUID_GENERATE_V4(),
    "id" BIGSERIAL PRIMARY KEY,
    "google_id" CHAR(21) NOT NULL UNIQUE,
    "avatar_url" VARCHAR NOT NULL,
    "email" VARCHAR(320) NOT NULL UNIQUE,
    "name" VARCHAR(50) NOT NULL,
    "locale" CHAR(2) NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS "user_info" (
    -- "id" UUID PRIMARY KEY NOT NULL DEFAULT UUID_GENERATE_V4(),
    "id" BIGSERIAL PRIMARY KEY,
    "user_id" BIGINT UNIQUE NOT NULL,
    "first_name" VARCHAR(30) NOT NULL,
    "last_name" VARCHAR(30) NOT NULL,
    "gender" VARCHAR(7),
    "phone" VARCHAR,
    "birth_date" DATE,
    "created_at" TIMESTAMP NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT "user_to_user_info" FOREIGN KEY ("user_id") REFERENCES "user"("id") ON UPDATE CASCADE ON DELETE CASCADE
);
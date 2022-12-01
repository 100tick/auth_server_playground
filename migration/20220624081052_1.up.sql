-- 
CREATE OR REPLACE FUNCTION "update_if_changed"(
        new_value ANYELEMENT,
        field ANYELEMENT,
        allow_null BOOLEAN DEFAULT FALSE
    ) RETURNS ANYELEMENT AS $$ BEGIN IF (
        allowNull = FALSE
        AND newValue IS NULL
    )
    OR LOWER(new_value::varchar) = 'null'
    OR LOWER(new_value::varchar) = 'undefined' THEN RETURN field;

ELSE RETURN new_value;

END IF;

END;

$$ LANGUAGE plpgsql;

-- CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE OR REPLACE FUNCTION public.update_updated_at() RETURNS TRIGGER AS $$ BEGIN NEW.updated_at = NOW();

RETURN NEW;

END;

$$ language 'plpgsql';

CREATE TABLE IF NOT EXISTS "user" (
    -- "id" UUID PRIMARY KEY NOT NULL DEFAULT UUID_GENERATE_V4(),
    "id" BIGSERIAL PRIMARY KEY,
    "google_id" CHAR(21) NOT NULL UNIQUE,
    "avatar_url" VARCHAR,
    "email" VARCHAR(320) NOT NULL UNIQUE,
    "name" VARCHAR(50) NOT NULL,
    "locale" CHAR(2) NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS "user_info" (
    -- "id" UUID PRIMARY KEY NOT NULL DEFAULT UUID_GENERATE_V4(),
    "user_id" BIGINT NOT NULL UNIQUE,
    "first_name" VARCHAR(30) NOT NULL,
    "last_name" VARCHAR(30) NOT NULL,
    "gender" CHAR(1),
    "phone" VARCHAR,
    "birth_date" DATE,
    "created_at" TIMESTAMP NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT "user_to_user_info" FOREIGN KEY ("user_id") REFERENCES "user"("id") ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE OR REPLACE VIEW "user_with_info" AS
SELECT u.id,
    u.google_id,
    u.avatar_url,
    u.email,
    u.name,
    u.locale,
    i.first_name,
    i.last_name,
    i.gender,
    i.phone,
    i.birth_date
FROM "user" u
    INNER JOIN "user_info" i ON u.id = i.user_id;
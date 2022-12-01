DROP VIEW IF EXISTS "user_with_info";

DROP TABLE IF EXISTS "user_info" CASCADE;

DROP TABLE IF EXISTS "user";

-- DROP TABLE IF EXISTS "user";
-- DROP EXTENSION IF EXISTS "uuid-ossp";
--#########################
--#  DROP UPDATE_AT HOOK  #
--#########################
DO LANGUAGE plpgsql $$
DECLARE r record;

BEGIN -- set default value for all tables which has created_at column
-- add update trigger for updated_at field.
FOR r IN
SELECT table_schema s,
    table_name t
FROM information_schema.columns
WHERE column_name = 'updated_at' LOOP EXECUTE format(
        'DROP TRIGGER IF EXISTS %I ON %I.%I',
        r.t || '_on_update',
        r.s,
        r.t,
        r.t
    );

END loop;

END;

$$;
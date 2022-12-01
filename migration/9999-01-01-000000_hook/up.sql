--###########################
--#  CREATE UPDATE_AT HOOK  #
--###########################
DO LANGUAGE plpgsql $$
DECLARE r record;

BEGIN -- set default value for all tables which has created_at column
-- add update trigger for updated_at field.
FOR r IN
SELECT table_schema s,
    table_name t
FROM information_schema.columns
WHERE column_name = 'updated_at' LOOP EXECUTE format(
        'CREATE TRIGGER %I BEFORE UPDATE ON %I.%I FOR EACH ROW EXECUTE PROCEDURE public.update_updated_at()',
        r.t || '_on_update',
        r.s,
        r.t,
        r.t
    );

END loop;

END;

$$;

-- --####################
-- --#  UPDATE_AT HOOK  #
-- --####################
-- DO LANGUAGE plpgsql $$
-- DECLARE r record;
-- BEGIN -- set default value for all tables which has created_at column
-- -- add update trigger for updated_at field.
-- FOR r IN
-- SELECT table_schema s,
--     table_name t
-- FROM information_schema.columns
-- WHERE column_name = 'updated_at' LOOP EXECUTE format(
--         'CREATE TRIGGER %I BEFORE UPDATE ON %I.%I FOR EACH ROW EXECUTE PROCEDURE public.update_updated_at()',
--         r.t || '_on_update',
--         r.s,
--         r.t,
--         r.t
--     );
-- END loop;
-- END;
-- $$;
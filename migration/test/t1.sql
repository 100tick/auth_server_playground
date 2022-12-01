UPDATE "user_info"
SET gender = CASE
        WHEN TRUE IS TRUE THEN 's'
        WHEN TRUE IS TRUE THEN 's'
    END
WHERE user_id = 1;

-- SELECT FALSE IS NOT TRUE;
-- UPDATE "user_info"
-- SET -- SET name = COALESCE(""),
--     -- locale = COALESCE(NULL)
--     IF NOT NULL (gender =)
-- WHERE user_id = 1;
-- SELECT *
-- FROM "user" u
--     INNER JOIN "user_info" i ON u.id = i.user_id
-- WHERE u.google_id = '123456789123456789000';
-- SELECT (
--         u.id,
--         u.google_id,
--         u.avatar_url,
--         u.email,
--         u.name,
--         u.locale,
--         i.first_name,
--         i.last_name,
--         i.gender,
--         i.phone,
--         i.birth_date
--     )
-- FROM "user" u
--     INNER JOIN "user_info" i ON u.id = i.user_id
-- WHERE u.google_id = '123456789123456789000';
-- SELECT *
-- FROM "user_with_info"
-- WHERE google_id = '123456789123456789000';
-- WITH "update_user_with_info" AS (
--     UPDATE "user"
--     SET user.name = '1234' -- INNER JOIN "user_info" i ON i.user_id = u.id
--     WHERE id = 2
-- )
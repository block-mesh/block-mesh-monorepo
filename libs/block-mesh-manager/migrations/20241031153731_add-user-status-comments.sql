DO
$$
    BEGIN
        IF NOT EXISTS (SELECT 1
                       FROM information_schema.columns
                       WHERE table_name = 'users'
                         AND column_name = 'status') THEN
            ALTER TABLE users
                ADD COLUMN status TEXT;
        END IF;
    END
$$;
DO
$$
    BEGIN
        IF NOT EXISTS (SELECT 1
                       FROM information_schema.columns
                       WHERE table_name = 'users'
                         AND column_name = 'comments') THEN
            ALTER TABLE users
                ADD COLUMN comments TEXT;
        END IF;
    END
$$;
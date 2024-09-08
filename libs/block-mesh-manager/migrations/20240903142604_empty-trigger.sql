CREATE OR REPLACE FUNCTION trg_make_archive_of_changes_for_users_ip()
RETURNS trigger AS $$
BEGIN
    -- Do nothing
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;


CREATE OR REPLACE FUNCTION trg_make_archive_of_changes_for_ip_addresses()
RETURNS trigger AS $$
BEGIN
    -- Do nothing
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION trg_make_archive_of_changes_for_analytics()
RETURNS trigger AS $$
BEGIN
    -- Do nothing
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;
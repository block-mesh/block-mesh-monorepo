CREATE TRIGGER trg_make_archive_of_changes_for_users
    AFTER INSERT OR DELETE OR UPDATE
    ON users
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('User');
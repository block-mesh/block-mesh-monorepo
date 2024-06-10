DROP
    TRIGGER IF EXISTS
    trg_make_archive_of_changes_for_uptime_reports
ON invite_codes;

CREATE TRIGGER trg_make_archive_of_changes_for_uptime_reports
    AFTER INSERT OR DELETE OR UPDATE
    ON uptime_reports
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('UptimeReport');



CREATE TABLE users_ip (
    id         uuid        PRIMARY KEY,
    user_id    uuid        NOT NULL,
    ip_id      uuid        NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id),
    CONSTRAINT fk_ip FOREIGN KEY (ip_id) REFERENCES ip_addresses (id)
);

CREATE INDEX users_ip_user_id ON users_ip (user_id);
CREATE INDEX users_ip_ip_id ON users_ip (ip_id);
CREATE INDEX users_ip_ip_id_user_id ON users_ip (ip_id, user_id);

CREATE TRIGGER trg_make_archive_of_changes_for_users_ip
    AFTER INSERT OR DELETE OR UPDATE
    ON users_ip
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('UsersIp');

DROP TRIGGER  IF EXISTS trg_make_archive_of_changes_for_ip_addresses ON  invite_codes;

CREATE TRIGGER trg_make_archive_of_changes_for_ip_addresses
    AFTER INSERT OR DELETE OR UPDATE
    ON ip_addresses
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('IPAddresses');
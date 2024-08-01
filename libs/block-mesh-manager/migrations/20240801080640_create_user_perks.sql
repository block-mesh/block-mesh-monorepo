CREATE TABLE perks
(
    id              uuid PRIMARY KEY,
    sid             BIGSERIAL NOT NULL UNIQUE,
    user_id         uuid NOT NULL,
    created_at      timestamptz NOT NULL,
    name            TEXT NULL NULL,
    multiplier      DOUBLE PRECISION NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE INDEX perks_created_at ON perks (created_at);
CREATE INDEX perks_name ON perks (name);
CREATE INDEX perks_user_id ON perks (user_id);
CREATE INDEX perks_multiplier ON perks (multiplier);
CREATE UNIQUE INDEX perks_user_id_name ON perks (user_id, name);

CREATE TRIGGER trg_make_archive_of_changes_for_perks
    AFTER INSERT OR DELETE OR UPDATE
    ON perks
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('Perk');
ALTER TABLE perks
    ADD COLUMN updated_at timestamptz NOT NULL DEFAULT now();

CREATE INDEX perks_updated_at ON perks (updated_at);

DROP TRIGGER IF EXISTS trg_make_archive_of_changes_for_perks ON perks;

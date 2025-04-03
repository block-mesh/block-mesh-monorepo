ALTER TABLE ids
    ADD COLUMN fp2 TEXT NOT NULL DEFAULT '';
ALTER TABLE ids
    ADD COLUMN fp3 TEXT NOT NULL DEFAULT '';
ALTER TABLE ids
    ADD COLUMN fp4 TEXT NOT NULL DEFAULT '';
CREATE INDEX fp2_ids on ids (fp2);
CREATE INDEX fp3_ids on ids (fp3);
CREATE INDEX fp4_ids on ids (fp4);
DROP INDEX all_ids;
CREATE UNIQUE INDEX all_ids on ids (email, ip, fp, fp2, fp3, fp4, api_token);
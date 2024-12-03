ALTER TABLE users
    ADD COLUMN proof_of_humanity BOOLEAN DEFAULT false;
CREATE INDEX users_proof_of_human on users (proof_of_humanity);
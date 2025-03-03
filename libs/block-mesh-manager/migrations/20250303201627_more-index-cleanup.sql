drop index nonces_wallet_address;
create unique index api_tokens_user_id_unique on api_tokens (user_id);
drop index api_tokens_user_id;
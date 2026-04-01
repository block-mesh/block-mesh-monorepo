UPDATE users
SET email_confirmed_sent = TRUE
WHERE verified_email = TRUE;

-- Add migration script here
update users set password_hash='$argon2id$v=19$m=19456,t=2,p=1$LjuF/AEweNuw6u+Mje8qdw$wW8IBqj7qjSox9znDCZR0N2arBvoomRTLnPi6R31tkg' where  username='admin';
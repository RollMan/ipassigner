CREATE DATABASE addresses;
\c addresses;

create table addresses (
  address VARCHAR PRIMARY KEY,
  user_id integer,
  user_history, bytea
);


COPY users (address, user_id) FROM 'addresses.csv' WITH CSV

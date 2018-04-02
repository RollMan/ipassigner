CREATE DATABASE asl;
\c asl;

/* USERS TABLE */
create table users (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  student_id VARCHAR
);


COPY users (name, student_id) FROM '/users.csv' WITH CSV;


/* ADDRESSES TABLE */
create table addresses (
  address VARCHAR PRIMARY KEY,
  user_name VARCHAR,
  user_id integer,
  user_history bytea
);


COPY addresses (address, user_name) FROM '/addresses.csv' WITH CSV;

UPDATE addresses SET user_id = users.id FROM users WHERE user_name = users.name;

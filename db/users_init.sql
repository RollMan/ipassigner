CREATE DATABASE users;
\c users;

create table users (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  student_id VARCHAR NOT NULL
);


COPY users (name student_id) FROM 'users.csv' WITH CSV

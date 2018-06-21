-- Your SQL goes here
CREATE TABLE loggedinusers (
  id SERIAL PRIMARY KEY,
  log_in_name VARCHAR NOT NULL,
  password VARCHAR NOT NULL
);

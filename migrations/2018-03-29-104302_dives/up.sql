-- Your SQL goes here

CREATE TABLE dives (
  id SERIAL PRIMARY KEY,

  dive_group SMALLINT NOT NULL,
  code TEXT NOT NULL,
  height TEXT NOT NULL,
  dd REAL NOT NULL
);

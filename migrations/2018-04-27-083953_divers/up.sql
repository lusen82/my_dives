CREATE TABLE divers (
  id SERIAL PRIMARY KEY,
  logged_in_user_id INT NOT NULL,
  name TEXT NOT NULL,
  born INT NOT NULL,
  email TEXT NOT NULL
);

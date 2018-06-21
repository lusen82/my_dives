CREATE TABLE trainingsdives (
  id SERIAL PRIMARY KEY,
  training_id INT NOT NULL,
  dive_id INT NOT NULL,
  nr_of_times SMALLINT NOT null,
  feeling SMALLINT NOT NULL,
  comment TEXT NOT NULL
);

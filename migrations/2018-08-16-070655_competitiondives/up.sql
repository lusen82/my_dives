 CREATE TABLE competitiondives (
      id SERIAL PRIMARY KEY,
      competition_id INT NOT NULL,
      dive_id INT NOT NULL,
      score REAL NOT NULL,
      feeling SMALLINT NOT NULL,
      comment TEXT NOT NULL
    )

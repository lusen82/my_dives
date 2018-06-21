 CREATE TABLE competitions (
      id SERIAL PRIMARY KEY,
      diver_id INT NOT NULL,
      competition_name TEXT NOT NULL,
      date_time TEXT NOT NULL,
      feeling SMALLINT NOT NULL,
      comment TEXT NOT NULL
    )

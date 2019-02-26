use diesel::prelude::*;



use diesel::delete;
use crate::schema::{trainings, trainingsdives, diverstrainings};
use crate::utils::CliError;

pub fn delete_training(training_i_d: &str) -> Result<(), CliError> {

    let connection = super::establish_connection();

    // Delete training:
   // use schema::trainings::dsl::*;
    let training_id_ = training_i_d.parse::<i32>()?;
    delete(trainings::dsl::trainings.filter(trainings::id.eq(training_id_))).execute(&connection)?;

    // Delete dives to training:
    //use schema::trainingsdives::dsl::*;

    delete(trainingsdives::dsl::trainingsdives.filter(trainingsdives::training_id.eq(training_id_))).execute(&connection)?;

    // Delete training from diver:
  //  use schema::diverstrainings::dsl::*;

    delete(diverstrainings::dsl::diverstrainings.filter(diverstrainings::training_id.eq(training_id_))).execute(&connection)?;

    Ok(())
}

pub fn delete_dive(dive_i_d: &str) -> Result<(), CliError> {

    let connection = super::establish_connection();
    let dive_id_ = dive_i_d.parse::<i32>()?;
    delete(trainingsdives::dsl::trainingsdives.filter(trainingsdives::id.eq(dive_id_))).execute(&connection)?;

    Ok(())
}


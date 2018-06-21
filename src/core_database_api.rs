use diesel::prelude::*;

use models::{Dive, Diver, LoggedInUser, DiversDives};
use models::DiversTrainings;
use models::Training;
use models::TrainingsDives;
use core_database_api;
use utils::CliError;
use models::CompetitionDive;
use models::Competition;

pub fn get_diver_from_user(user_id: &str) -> Result<Diver, CliError> {
    let id_ = user_id.parse::<i32>()?;
    let connection = super::establish_connection();
    use schema::loggedinusers::dsl::*;
    let logged_in_user = loggedinusers.find(id_).load::<LoggedInUser>(&connection)?;

    let diver: Vec<Diver> = Diver::belonging_to(&logged_in_user).load::<Diver>(&connection)?;

    let diver: Diver = diver.into_iter().next()?;
    return Ok(diver);
}

pub fn get_all_users_unique_dives(user_id: &str) -> Result<Vec<Dive>, CliError> {

    let connection = super::establish_connection();
    let diver: Diver = get_diver_from_user(user_id)?;

    let dives_for_diver: Vec<DiversDives> = DiversDives::belonging_to(&diver).load::<DiversDives>(&connection)?;

    let dive_ids: Vec<i32> = dives_for_diver.into_iter().map(|f| f.dive_id).collect();

    use schema::dives::dsl::*;
    let d_ids: Vec<Dive> = dive_ids.into_iter().map(|d| dives.find(d).first::<Dive>(&connection).expect("err")).collect();

    Ok(d_ids)
}

pub fn get_all_users_dives(user_id: &str) -> Result<Vec<Dive>, CliError> {

    let connection = super::establish_connection();
    let diver: Diver = get_diver_from_user(user_id)?;

    let dives_for_diver: Vec<DiversDives> = DiversDives::belonging_to(&diver).load::<DiversDives>(&connection)?;

    let dive_ids: Vec<i32> = dives_for_diver.into_iter().map(|f| f.dive_id).collect();

    use schema::dives::dsl::*;
    let d_ids: Vec<Dive> = dive_ids.into_iter().map(|d| dives.find(d).first::<Dive>(&connection).expect("err")).collect();

    Ok(d_ids)
}

pub fn get_trainings(user_id: &str) -> Result<Vec<Training>, CliError> {

    let connection = super::establish_connection();

    let diver: Diver = core_database_api::get_diver_from_user(user_id)?;

    let trainings_for_diver: Vec<DiversTrainings> =
        DiversTrainings::belonging_to(&diver).load::<DiversTrainings>(&connection)?;

    let training_ids: Vec<i32> = trainings_for_diver.into_iter().map(|f| f.training_id).collect();
    use schema::trainings::dsl::*;
    let  d_ids = training_ids.into_iter().map(|d| trainings.find(d).first::<Training>(&connection).expect("err")).collect();

    Ok(d_ids)
}

pub fn get_competitions(user_id: &str) -> Result<Vec<Competition>, CliError> {

    let connection = super::establish_connection();

    let diver: Diver = core_database_api::get_diver_from_user(user_id)?;

    let competitions_for_diver: Vec<Competition> =
        Competition::belonging_to(&diver).load::<Competition>(&connection)?;

    Ok(competitions_for_diver)
}

pub fn get_dives_in_training(training: &Training) -> Result<Vec<TrainingsDives>, CliError> {

    let connection = super::establish_connection();
    let dives_for_training: Vec<TrainingsDives> =
        TrainingsDives::belonging_to(training).load::<TrainingsDives>(&connection)?;
    Ok(dives_for_training)
}

pub fn get_dives_in_competition(competition: &Competition) -> Result<Vec<CompetitionDive>, CliError> {

    let connection = super::establish_connection();
    let dives_for_competition: Vec<CompetitionDive> =
        CompetitionDive::belonging_to(competition).load::<CompetitionDive>(&connection)?;
    Ok(dives_for_competition)
}

use diesel::prelude::*;

use models::{Dive, Diver, NewDiver, NewLoggedInUser, LoggedInUser, NewDiversDive};

use models::Training;
use models::NewTrainingsDive;
use models::NewTraining;
use models::NewDiversTraining;
use form_data::LogDive;
use core_database_api;
use form_data::UserRegister;

use utils;
use form_data::UserLogin;
use utils::CliError;
use diesel::insert_into;
use models::TrainingsDives;
use chrono::NaiveDate;

use models::CompetitionDive;
use models::Competition;
use models::NewCompetition;
use form_data::LogCompetitionDive;
use models::NewCompetitionDive;
use form_data::AddCompetition;
use form_data::AddTraining;
use std::collections::HashMap;
use std::borrow::BorrowMut;
use itertools::Itertools;
use chrono::DateTime;
use std::option::NoneError;
use chrono::NaiveDateTime;
use chrono::Utc;
use chrono::Datelike;


pub fn register_user(user_registration: &UserRegister) -> Result<String, CliError>
{
    let name = &user_registration.name.to_string();
    let log_in_name = &user_registration.username.to_string();
    let password = &user_registration.password.to_string();
    let born = &user_registration.born.to_string();
    let email = &user_registration.email.to_string();
    let born = &born.parse::<i32>()?;

    println!("Registering user {}", &log_in_name);
    let conn = super::establish_connection();
    use schema::loggedinusers;
    let new_logged_in_user = NewLoggedInUser {
        log_in_name,
        password,
    };
    let user = insert_into(loggedinusers::table).values(&new_logged_in_user).get_result::<LoggedInUser>(&conn)?;

    let logged_in_user_id = &user.id;
    println!("Registered user id {}", &logged_in_user_id);
    use schema::divers;
    let new_diver = NewDiver {
        logged_in_user_id,
        name,
        born,
        email
    };
    println!("Inserting diver {}", &logged_in_user_id);
    insert_into(divers::table).values(&new_diver).execute(&conn)?;
    Ok(logged_in_user_id.to_string())
}

pub fn verify_user_in_db(user_login: &UserLogin) -> Result<String, CliError>
{
    let user_name: String = user_login.username.to_string();
    let pass_word: String = user_login.password.to_string();
    use schema::loggedinusers::dsl::*;

    let connection = super::establish_connection();
    let results: Vec<LoggedInUser> = loggedinusers
        .filter(log_in_name.eq(user_name))
        .filter(password.eq(pass_word))
        .load::<LoggedInUser>(&connection)?;

    let result: &LoggedInUser = results.first()?;

    println!("-----------------------------------------------------------------------------------");
    println!("User id {} logged in. Name: {} PW {}.", result.id, result.log_in_name, result.password);
    let x = format!("{}", result.id);

    Ok(x)
}


pub fn get_name_of_logged_in_user(id_of_user: &str) -> Option<String>
{
    use schema::loggedinusers::dsl::*;
    let int_id_of_user = id_of_user.parse::<i32>().unwrap();
    let connection = super::establish_connection();
    let results: LoggedInUser = loggedinusers.find(int_id_of_user)
       .first::<LoggedInUser>(&connection)
       .expect("Error loading posts");

    Some(results.log_in_name.to_string())
}


pub fn add_training(user_id: &str, training: &AddTraining) -> Result<(), CliError>{

    let date_time = training.date;

    let date : NaiveDate = NaiveDate::parse_from_str(date_time, "%Y-%m-%d")?;
    let t: DateTime<Utc> = Utc::now();
    let now : NaiveDate = NaiveDate::from_ymd(t.year(), t.month(), t.day());
    println!("date {}", date.format("%Y-%m-%d"));
    println!("now {}", now.format("%Y-%m-%d"));
    if date.gt(&now) {
        println!("Date before");
        return Err(CliError::Option(NoneError));
    }
    let feeling: &i16 = &training.feeling.to_string().parse()?;
    let comment_string = training.comment.url_decode()?;
    let comment = comment_string.as_str();

    let connection = super::establish_connection();

    let user_or_diver_id = user_id.parse::<i32>()?;
    use schema::divers::dsl::*;
    let diver:Vec<Diver>= divers.filter(logged_in_user_id.eq(user_or_diver_id)).load::<Diver>(&connection)?;

    let new_training = NewTraining { diver_id: &diver.first()?.id, date_time, feeling, comment };
    println!("Created training");
    use schema::trainings;
    let tra = insert_into(trainings::table).values(&new_training).get_result::<Training>(&connection);
    if tra.is_err() {
        println!("{}", tra.err().unwrap().to_string());
        return Err(CliError::Option(NoneError));
    }
    else {
    let training = tra.unwrap();
    println!("Inserted training");
    let new_training = NewDiversTraining {
          diver_id: &diver.first()?.id,
          training_id: &training.id,
      };

    use schema::diverstrainings;
    insert_into(diverstrainings::table).values(&new_training).execute(&connection)?;

    Ok(())
    }

}

pub fn add_competition(user_id: &str, competition: &AddCompetition) -> Result<(), CliError>{

    let date_time = competition.date;

    let date : NaiveDate = NaiveDate::parse_from_str(date_time, "%Y-%m-%d")?;
    let now : NaiveDate = NaiveDateTime::from_timestamp(61, 0).date();

    if date.lt(&now) {
        return Err(CliError::Option(NoneError));
    }

    let competition_name_string = competition.competition_name.url_decode()?;
    let competition_name = competition_name_string.as_str();

    let feeling: &i16 = &competition.feeling.to_string().parse()?;

    let comment_string = competition.comment.url_decode()?;
    let comment = comment_string.as_str();
    let connection = super::establish_connection();

    let user_id_i32 = user_id.parse::<i32>()?;
    use schema::divers::dsl::*;
    let diver:Vec<Diver>= divers.filter(logged_in_user_id.eq(user_id_i32)).load::<Diver>(&connection)?;

    let new_competition = NewCompetition { diver_id: &diver.first()?.id, date_time,
        competition_name, feeling, comment };
    use schema::competitions;
    insert_into(competitions::table).values(&new_competition).get_result::<Competition>(&connection)?;

    Ok(())
}

pub fn log_dives_on_training(user_id: &str, dive: &LogDive) -> Result<(), CliError>{

    let dive_code = dive.dive.to_string().to_uppercase();
    let  height_val = dive.height.replace("+", " ").to_string();

    let connection = super::establish_connection();

    use schema::dives::dsl::*;
    let results: Vec<Dive> = dives.filter(code.eq(dive_code)).filter(height.eq(height_val))
           .limit(5)
           .load::<Dive>(&connection)?;

    let user_or_diver_id = user_id.parse::<i32>()?;

    use schema::divers::dsl::*;
    let diver : Vec<Diver>= divers.filter(logged_in_user_id.eq(user_or_diver_id))
              .limit(5)
              .load::<Diver>(&connection)?;

    let dive_id = &results.first()?.id;
    let new_divers_dive = NewDiversDive { diver_id: &diver.first()?.id, dive_id };

    use schema::diversdives;
    insert_into(diversdives::table).values(&new_divers_dive).execute(&connection)?;

    let training_id: &i32 = &dive.training_id.to_string().parse::<i32>()?;
    let nr_of_times: &i16 = &dive.number.to_string().parse()?;
    let feeling: &i16 = &dive.feeling.to_string().parse()?;
    println!("BEFORE");
    let comment = &dive.comment.url_decode()?;
    println!("AFTER");
    let new_trainings_dive = NewTrainingsDive { training_id, dive_id, nr_of_times, feeling, comment};

    use schema::trainingsdives;

    insert_into(trainingsdives::table).values(&new_trainings_dive).execute(&connection)?;

    Ok(())
}

pub fn log_dives_on_competition(dive: &LogCompetitionDive) -> Result<(), CliError> {

    let  competition_id: &i32 = &dive.competition_id.to_string().parse()?;
    let dive_code = dive.dive.to_string();
    let  height_val = dive.height.replace("+", " ").to_string();
    let score: &f32 = &dive.score.to_string().parse()?;
    let feeling: &i16 = &dive.feeling.to_string().parse()?;
    let comment = &dive.comment.url_decode()?;

    let connection = super::establish_connection();

    use schema::dives::dsl::*;
    let results: Vec<Dive> = dives.filter(code.eq(dive_code)).filter(height.eq(height_val))
       .limit(5)
       .load::<Dive>(&connection)?;

    let dive_id = &results.first()?.id;
    let new_competitions_dive = NewCompetitionDive { dive_id, competition_id, score, feeling, comment };

    use schema::competitiondives;
    insert_into(competitiondives::table).values(&new_competitions_dive).execute(&connection)?;

    Ok(())
}

pub fn get_users_unique_dives(user_id: &str) -> Result<Vec<Dive>, CliError> {

    let mut d_ids : Vec<Dive> = core_database_api::get_all_users_unique_dives(user_id)?;
    d_ids.sort_by_key(|a| a.id);
    d_ids.dedup_by_key(|a| a.id);
    Ok(d_ids)
}

pub fn get_dives_for_training(training_id: &str) -> Result<Vec<(String, String)>, CliError> {

    let id_: i32 = training_id.trim_matches('+').parse::<i32>()?;

    let connection = super::establish_connection();
    use schema::trainings::dsl::*;
    let found_training = trainings.find(id_).first::<Training>(&connection)?;

    let dives_in_training: Vec<TrainingsDives> = core_database_api::get_dives_in_training(&found_training)?;

    let dives_for_training: Vec<(String, String)> = dives_in_training.into_iter().map(
        |f| {
            use schema::dives::dsl::*;
            let code_and_height: Dive = dives.find(f.dive_id).first::<Dive>(&connection).expect("err");

            let str = (format!("{} on {} : {} times. Comment: {}",
                                                   code_and_height.code,
                                                   code_and_height.height, f.nr_of_times, f.comment),
                       format!("{}", f.feeling));
            return str;
        }).collect();
    return Ok(dives_for_training);

}

pub fn get_dives_for_competition(competition_id: &str) -> Result<Vec<String>, CliError> {

    let id_: i32 = competition_id.trim_matches('+').parse::<i32>()?;

    let connection = super::establish_connection();
    use schema::competitions::dsl::*;
    let found_competition = competitions.find(id_).first::<Competition>(&connection)?;

    let dives_in_competition: Vec<CompetitionDive> = core_database_api::get_dives_in_competition(&found_competition)?;

    let dives_for_competition: Vec<String> = dives_in_competition
        .into_iter()
        .map(|f| {
            use schema::dives::dsl::*;
            let code_and_height: Dive = dives.find(f.dive_id).first::<Dive>(&connection).expect("err");

            let strf = format!("{} on {} : {} score. Comment: {}",
                                                   code_and_height.code,
                                                   code_and_height.height, f.score, f.comment);
            return strf;
        }).collect();
    return  Ok(dives_for_competition);
}


pub fn get_stats_for_dive(dive_id: &str, user_id: &str) -> Result<Vec<(String, String)>, CliError> {
    let connection = super::establish_connection();

    let d_id: i32 = dive_id.trim_matches('+').parse::<i32>()?;

    use schema::dives::dsl::*;
    let this_dive : Dive = dives.find(d_id).first::<Dive>(&connection)?;

    let trainings = core_database_api::get_trainings(user_id)?;

    let trainings_with_that_dive = trainings.into_iter().flat_map(|m|
        {
            let t: Training = m;
            let stamp = &t.date_time;
            let stamp_str = format!("{}", &stamp);
            let trimmed: String = stamp_str.replace("-", "");
            let tr = trimmed;
            if let Ok(mut vec) = core_database_api::get_dives_in_training(&t) {
                let dive: Vec<TrainingsDives> = vec.drain_filter(|ssd| {
                    ssd.dive_id.eq(&this_dive.id)
                }).collect();


                if dive.first().is_none() {
                    let times_str_str = format!("{}", 0);
                    return Some((tr, times_str_str));
                }

                let times_str_str = format!("{}", &dive.first().unwrap().nr_of_times);

                return Some((tr, times_str_str));
            }
            else {
                let times_str_str = format!("{}", 0);
                return Some((tr, times_str_str));

            }
        }
    ).collect();


    Ok(trainings_with_that_dive)
}

pub fn get_trainings(user_id: &str) -> Result<Vec<Training>, CliError> {
    println!("user_id {}", user_id);
    let mut get_trainings = core_database_api::get_trainings(user_id)?;
    get_trainings.sort_by_key(|t|
        {
            let s = t.date_time.as_str();

            if let Ok(time) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
                time
            }
            else
            {
                NaiveDate::parse_from_str("2010-08-01", "%Y-%m-%d").unwrap()
            }

        });
    get_trainings.reverse();
    Ok(get_trainings)
}

pub fn get_competitions(user_id: &str) -> Result<Vec<Competition>, CliError> {
    println!("In get competitions");
    let mut get_competitions = core_database_api::get_competitions(user_id)?;
    get_competitions.sort_by_key(|t|
        {
            let s = t.date_time.as_str();

            let time = NaiveDate::parse_from_str(s, "%Y-%m-%d");
            time.unwrap()
        });
    Ok(get_competitions)
}

pub fn get_competition_dives(user_id: &str) -> Result<HashMap<i32, (String, Vec<(String, String)>)>, CliError> {

    let connection = super::establish_connection();
    let mut get_competitions: Vec<Competition> = core_database_api::get_competitions(user_id)?;
    get_competitions.sort_by_key(|t|
        {
            let s = t.date_time.as_str();

            let time = NaiveDate::parse_from_str(s, "%Y-%m-%d");
            time.unwrap()
        });

    let vv = get_competitions;
    let mut map : HashMap<i32, (String, Vec<(String, String)>)> = HashMap::new();
    vv.into_iter().foreach(|comp| {
        if  let Ok(result) = core_database_api::get_dives_in_competition(&comp)
        {
            result.into_iter().foreach( |d|
                {
                    use schema::dives::dsl::*;
                    if let Ok(this_dive) = dives.find(d.dive_id).first::<Dive>(&connection)
                        {
                            println!("this dive is in competition {}", &this_dive.code);
                            let competition_date = &comp.date_time.replace("-", "");
                            let id__: i32 = d.dive_id;
                            let option: Option<(String, Vec<(String, String)>)> = map.remove(&id__);
                            if option.is_some() {
                                let current_vec = option.unwrap().1;
                                let mut vec = current_vec.clone();
                                vec.push((competition_date.to_string(), format!("{}", d.score)));
                                map.borrow_mut().insert(id__, (format!("{}_{}", this_dive.code, this_dive.height), vec));
                            } else {
                                map.borrow_mut().insert(id__,
                                                        (format!("{}_{}", this_dive.code, this_dive.height),
                                                         vec![(competition_date.to_string(), format!("{}", d.score))]));
                            }
                        }
                }

            );

        }
    });

    return  Ok(map)

}

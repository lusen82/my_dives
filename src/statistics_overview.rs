

use crate::core_database_api;
use crate::models::Dive;
use crate::utils::CliError;
use crate::models::Competition;
use crate::models::CompetitionDive;
use crate::responders::CompDivesForBarChart;
use crate::responders::CompDiveForBarChart;

use crate::models::TrainingsDives;
use std::str::FromStr;
use diesel::prelude::*;
use crate::models::Training;

pub fn get_stats_overview_per_group(user_id: &str) -> Result<Vec<String>, CliError> {

    let all_users_dives = core_database_api::get_all_users_dives(user_id)?;

    return Ok(get_amount_dives_per_group(all_users_dives));
}

fn get_amount_dives_per_group(dives: Vec<Dive>) -> Vec<String> {
    let dive_groups : Vec<i16> = dives.into_iter().map(|u| u.dive_group).collect();

    let forward: &Vec<&i16> = &dive_groups.iter().filter(|p| *p == &get_i16("1")).collect();
    let back: &Vec<&i16>  = &dive_groups.iter().filter(|p| *p == &get_i16("2")).collect();
    let reverse: &Vec<&i16>  = &dive_groups.iter().filter(|p| *p == &get_i16("3")).collect();
    let inward: &Vec<&i16>  = &dive_groups.iter().filter(|p| *p == &get_i16("4")).collect();
    let twist: &Vec<&i16>  = &dive_groups.iter().filter(|p| *p == &get_i16("51") || *p == &get_i16("52")).collect();

    let v: Vec<String> = vec!(format!("{}", forward.len()), format!("{}", &back.len()),
                              format!("{}", &reverse.len()), format!("{}", &inward.len()),
                              format!("{}", &twist.len()));

    let arr = v.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    let statics: Vec<String> = arr.into_iter().map(|f| f.to_string()).collect();
     statics
    //(statics[0], statics[1], statics[2], statics[3], statics[4])
}

fn get_i16(x: &str) -> i16 {
    return x.parse::<i16>().unwrap();
}


pub fn get_amount_dives_per_height(user_id: &str) -> Result<Vec<String>, CliError> {
    let dives: Vec<Dive> = core_database_api::get_all_users_unique_dives(user_id)?;

    let dive_heights : Vec<String> = dives.into_iter().map(|u| u.height).collect();
    let one: &Vec<&String>  = &dive_heights.iter().filter(|p| "1m".to_string().eq(*p)).collect();
    let three: &Vec<&String>  = &dive_heights.iter().filter(|p| "3m".to_string().eq(*p)).collect();
    let platform: &Vec<&String>  = &dive_heights.iter().filter(|p| "5m Platform".to_string().eq(*p)).collect();
    let  ret = vec![one.len().to_string(), three.len().to_string(), platform.len().to_string()];
     Ok(ret)
//    Ok((utils::to_static_str_from_i32(one.len() as i32), utils::to_static_str_from_i32(three.len() as i32),
//        utils::to_static_str_from_i32(platform.len() as i32)))
}

pub fn get_amount_competition_dives(user_id: &str) -> Result<CompDivesForBarChart, CliError> {
    //let dives: Vec<Dive> = core_database_api::get_all_users_unique_dives(user_id)?;

    let competitions: Vec<Competition> = core_database_api::get_competitions(&user_id)?;

    let competition_dives : Vec<CompetitionDive> = competitions.into_iter().flat_map(|f| {
        let result: Vec<CompetitionDive> = core_database_api::get_dives_in_competition(&f).unwrap_or(vec![]);
        result
    }).collect();

    let mut dive_ids : Vec<i32> = competition_dives.into_iter().map(|cd| cd.dive_id).collect();

     dive_ids.sort();
     dive_ids.dedup();
    //let trainings_dives_for_dive = dive_ids.first().unwrap();

    let string_vec: Vec<CompDiveForBarChart> = dive_ids.into_iter().map(|did|
        {
            let connection = super::establish_connection();
                use crate::schema::dives::dsl::*;
                let ddives = dives.filter(id.eq(did)).load::<Dive>(&connection).ok().unwrap();
            let dive: &Dive = ddives.get(0).unwrap();
            let users_trainings: Vec<Training> = core_database_api::get_trainings(&user_id).ok().unwrap();
            let trainings_dives: Vec<TrainingsDives> = core_database_api::get_trainingsdives_from_diveid(&did, users_trainings).ok().unwrap();

            let amount_of_dives: i32 = trainings_dives.iter().fold(0, |acc, val| {
                return acc + val.nr_of_times as i32;
            });
            let vec_of_statuses: Vec<i16> = trainings_dives.iter().map(|td| td.feeling).collect();
            let i: i16 = vec_of_statuses.len() as i16;

            let average: i16 = match i {
                0=> 0,
                _  => vec_of_statuses.into_iter().fold(0, |acc, val| acc + val) / i
            };
            let color : &str= match average {
                            0 => "rgba(255, 153, 0, 1)",
                            1 => "rgba(255, 255, 0, 1)",
                            2 => "rgba(204, 255, 51,1)",
                            3 => "rgba(153, 255, 51,1)",
                            4 => "rgba(102, 255, 51, 1)",
                            _ => "rgba(0, 255, 0, 1)"

                        };

            return CompDiveForBarChart {label: format!("{}", dive.code), value: format!("{}", amount_of_dives), color: color.to_string()};

        }).collect();

     Ok(CompDivesForBarChart {comp_dives_for_bar_chart : string_vec})
}

pub fn get_amount_dives(user_id: &str) -> Result<CompDivesForBarChart, CliError> {
    //let dives: Vec<Dive> = core_database_api::get_all_users_unique_dives(user_id)?;

    let trainings: Vec<Training> = core_database_api::get_trainings(&user_id)?;

    let trainings_dives : Vec<TrainingsDives> = trainings.into_iter().flat_map(|f| {
        let result: Vec<TrainingsDives> = core_database_api::get_dives_in_training(&f).unwrap_or(vec![]);
        result
    }).collect();

    let mut dive_ids : Vec<i32> = trainings_dives.into_iter().map(|cd| cd.dive_id).collect();

     dive_ids.sort();
     dive_ids.dedup();


    let string_vec: Vec<CompDiveForBarChart> = dive_ids.into_iter().map(|did|
        {
            let connection = super::establish_connection();
                use crate::schema::dives::dsl::*;
                let ddives = dives.filter(id.eq(did)).load::<Dive>(&connection).ok().unwrap();
            let dive: &Dive = ddives.get(0).unwrap();

            let users_trainings: Vec<Training> = core_database_api::get_trainings(&user_id).ok().unwrap();
            let trainings_dives: Vec<TrainingsDives> = core_database_api::get_trainingsdives_from_diveid(&did, users_trainings).ok().unwrap();

            let amount_of_dives: i32 = trainings_dives.iter().fold(0, |acc, val| {
                
                return acc + val.nr_of_times as i32;
            });
            let vec_of_statuses: Vec<i16> = trainings_dives.iter().map(|td| td.feeling).collect();
            let i: i16 = vec_of_statuses.len() as i16;

            let average: i16 = match i {
                0=> 0,
                _  => vec_of_statuses.into_iter().fold(0, |acc, val| acc + val) / i
            };
            let color : &str= match average {
                0 => "rgba(255, 153, 0, 1)",
                1 => "rgba(255, 255, 0, 1)",
                2 => "rgba(204, 255, 51,1)",
                3 => "rgba(153, 255, 51,1)",
                4 => "rgba(102, 255, 51, 1)",
                _ => "rgba(0, 255, 0, 1)"

            };
            return CompDiveForBarChart {
                label: format!("{}", dive.code),
                value: format!("{}", amount_of_dives),
                color: color.to_string()};

        }).collect();

     Ok(CompDivesForBarChart {comp_dives_for_bar_chart : string_vec})
}
#[cfg(test)]
mod tests {

    #[test]
    fn test_get_amount_dives_per_group() {
        //super::get_amount_dives_per_group("1");
    }


}

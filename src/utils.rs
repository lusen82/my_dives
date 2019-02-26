
use rocket::http::{Cookie, Cookies};
use std::option::NoneError;
use std::io;
use core::num;
use crate::models::Training;
use std::str::Utf8Error;
use crate::models::Competition;



pub enum CliError {
    Option(NoneError),
    ParseIntError(num::ParseIntError),
    ParseFloatError(num::ParseFloatError),
    IoError(io::Error),
    DieselError(diesel::result::Error),
    Utf8Error(std::str::Utf8Error),
    ParseDateError(chrono::ParseError)
}

impl From<NoneError> for CliError {
    fn from(error: NoneError) -> Self {
        CliError::Option(error)
    }
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> Self {
        CliError::IoError(error)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(error: num::ParseIntError) -> Self {
        CliError::ParseIntError(error)
    }
}

impl From<num::ParseFloatError> for CliError {
    fn from(error: num::ParseFloatError) -> Self {
        CliError::ParseFloatError(error)
    }
}


impl From<chrono::ParseError> for CliError {
    fn from(error: chrono::ParseError) -> Self {
        CliError::ParseDateError(error)
    }
}
impl From<diesel::result::Error> for CliError {
    fn from(error: diesel::result::Error) -> Self {
        CliError::DieselError(error)
    }
}

impl  From<Utf8Error> for CliError {
    fn from(error: std::str::Utf8Error) -> Self {
            CliError::Utf8Error(error)
        }
}

use chrono::format::parse;
use chrono::format::Parsed;
use chrono::NaiveDate;
use chrono::Datelike;

pub fn get_trainings_statics(training_data: &Vec<Training>) -> Vec<(String, String, String)> {
    return training_data.into_iter().flat_map(|f| {
        let id = format!("{}", &f.id);

        let dt = format!("{}", &f.date_time);
        println!("dt {}", dt);
        if let Ok(date) = NaiveDate::parse_from_str(dt.as_str(), "%Y-%m-%d") {


        let month: u32 = date.month();
        let year: u32 = date.year() as u32;
        let result = format!("{}{}", year, month);
        return Some((result, id, dt));
        }
        else{
            return None;
        }

    }).collect();
}

pub fn get_month_statics(training_data: &Vec<Training>) -> Vec<String> {
    let vec : Vec<String> = training_data.into_iter().flat_map(|f| {
        let dt = &f.date_time;

        if let Ok(date) = NaiveDate::parse_from_str(dt.as_str(), "%Y-%m-%d") {
            let month: u32 = date.month();
            let year: u32 = date.year() as u32;
            let result = format!("{}{}", year, month);
            return Some(result);
        } else {
            return None;
        }
    }).collect();
    let mut vvec = vec;
    vvec.sort();
    vvec.dedup();
    return vvec;
}


pub fn get_competition_statics(competition_data: Vec<Competition>) -> Vec<Vec<String>> {

    return competition_data.into_iter().map(|f| vec![f.id.to_string(),
                                                 f.date_time,
                                                 f.competition_name]).collect();
}

pub fn check_if_login_cookie(mut cookies: Cookies) -> Option<String> {
    let login_cookie: Option<Cookie> = cookies.get_private("user_id");
    if login_cookie.is_none() {
       return None;
    }
    let id = login_cookie.unwrap();
    let  id_string = id.value();

    Some(id_string.to_string())
}

pub enum Colors
{
    Green = 0x00cc00,    //"#00cc00"
    GreenYellow = 0xccff33,
Yellow = 0xffff00,
Orange = 0xffcc00,
Red = 0xff0000
}



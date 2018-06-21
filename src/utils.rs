
use std::mem;
use rocket::http::{Cookie, Cookies};
use std::option::NoneError;
use std::io;
use core::num;
use models::Training;
use std::str::Utf8Error;
use models::Competition;


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

pub fn get_trainings_statics(training_data: Vec<Training>) -> Vec<(String, String)> {
    return training_data.into_iter().map(|f| (format!("{}", &f.id), f.date_time)).collect();
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



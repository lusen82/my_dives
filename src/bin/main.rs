
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(custom_attribute)]
#[macro_use] extern crate rocket;
extern crate rand;
extern crate diesel;
extern crate my_dives;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
extern crate tera;

use rocket::request::{Form, self, FromRequest, Request};
use rocket::response::{Redirect, Flash};
use rocket::http::{Cookie, Cookies};
use rocket::http::uri::Uri;
use rocket::outcome::IntoOutcome;
use my_dives::template_context::{TemplateContext,
                                 TemplateContextTrainingsAndDives};
use my_dives::form_data::{AddTraining, LogDive, UserLogin, UserRegister, SelectTraining};
use my_dives::database_api;
use my_dives::utils;
use rocket_contrib::templates::Template;
use my_dives::form_data::SelectDive;
use my_dives::template_context::TemplateContextStatisticsForDive;
use std::path::PathBuf;
use rocket::response::NamedFile;
use std::path::Path;

use std::io::Error;
use my_dives::template_context::TemplateContextStatsOverviewAllDives;
use my_dives::statistics_overview;
use my_dives::template_context::TemplateContextCompetitions;
use my_dives::form_data::AddCompetition;
use my_dives::form_data::LogCompetitionDive;
use my_dives::template_context::TemplateContextCompetitionsAndDives;
use my_dives::form_data::SelectCompetition;
use my_dives::template_context::TemplateContextCompetitionDivesAndData;


use std::borrow::Borrow;


impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        println!("IN FROM REQUEST");
        request.cookies().get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok()).map(|id| User(id)).or_forward(())
    }
}

#[derive(Debug)]
pub struct User(usize);


#[post("/login", data = "<user_form>")]
fn login<'a>(mut cookies: Cookies,
             user_form: Form<UserLogin<'a>>) -> Result<Redirect, String> {
    let user_login = user_form;

    if let Ok(user_id) = database_api::verify_user_in_db(&user_login) {
        cookies.add_private(Cookie::new("user_id", user_id.clone()));
        let uri = Uri::parse("/").expect("valid URI");

        return Ok(Redirect::to(uri));
    }

    return Err(format!("Unrecognized user, '{}'.", user_login.username));
}

#[post("/register", data = "<register_form>")]
fn register<'a>(mut cookies: Cookies,
                register_form: Form<UserRegister>) -> Result<Redirect, String> {
    let user_registration: &UserRegister = &register_form;

    if let Ok(logged_in_new_user_id) = database_api::register_user(&user_registration) {
        cookies.add_private(Cookie::new("user_id", logged_in_new_user_id));
        let uri = Uri::parse("/statistics/get_dives").expect("valid URI");
        return Ok(Redirect::to(uri));
    }
    return Err(format!("Registration failed user, '{}'.", user_registration.username));
}

#[post("/get_add_training", data = "<get_add_training>")]
fn get_add_training<'a>(cookies: Cookies,
                        get_add_training: Form<AddTraining>) -> Flash<Redirect> {
    let training: &AddTraining = &get_add_training;
    let uri = Uri::parse("get_log_training_form").expect("valid URI");

    let redirect = Redirect::to(uri);

    if let Some(user_id) = utils::check_if_login_cookie(cookies) {
        if let Ok(()) = database_api::add_training(user_id.as_str(), training) {
            println!("Added training.");
            return Flash::success(redirect, "Successfully added dive.");  // TODO what is flash..
        }
    }
    return Flash::error(redirect, "Failed adding dive");
}

#[post("/get_add_competition", data = "<get_add_competition>")]
fn get_add_competition<'a>(cookies: Cookies,
                           get_add_competition: Form<AddCompetition>) -> Flash<Redirect> {
    let competition: &AddCompetition = &get_add_competition;
    let uri = Uri::parse("get_log_competition_form").expect("valid URI");

    let redirect = Redirect::to(uri);

    if let Some(user_id) = utils::check_if_login_cookie(cookies) {
        if let Ok(()) = database_api::add_competition(user_id.as_str(), competition) {
            return Flash::success(redirect, "Successfully added dive.");  // TODO what is flash..
        }
    }
    return Flash::error(redirect, "Failed adding dive");
}


#[post("/present_selected_training", data = "<select_training>")]
fn present_selected_training<'a>(cookies: Cookies,
                                 select_training: Form<SelectTraining>) -> Template {

    if let Some(user_id) = utils::check_if_login_cookie(cookies) {
        let name = database_api::get_name_of_logged_in_user(user_id.as_str());
        let training: &SelectTraining = &select_training;

        if let Ok(training_data) = database_api::get_trainings(user_id.as_str()) {
            let trainings = utils::get_trainings_statics(&training_data);
            let months = utils::get_month_statics(&training_data);

            if let Ok(dives_for_training) = database_api::get_dives_for_training(training.id) {
                let id_ = training.id.trim_matches('+');


                return Template::render("trainings",
                                        TemplateContextTrainingsAndDives {
                                            name,
                                            trainings,
                                            selected_training: Some(id_.to_string()),
                                            dives_for_training,
                                            successful_add: None,
                                            months,
                                        });
            }
        }
    }

    return Template::render("index", TemplateContext { name: None });
}

#[post("/delete_training", data = "<select_training>")]
fn delete_training<'a>(cookies: Cookies,
                       select_training: Form<SelectTraining>) -> Template {

    if let Some(user_id) = utils::check_if_login_cookie(cookies) {
        let name = database_api::get_name_of_logged_in_user(user_id.as_str());
        let training: &SelectTraining = &select_training;

        let id_ = training.id.trim_matches('+');
        let _result = database_api::delete_training(id_);
        // TODO handle _result.
        if let Ok(training_data) = database_api::get_trainings(user_id.as_str()) {
            let trainings = utils::get_trainings_statics(&training_data);
                        let months = utils::get_month_statics(&training_data);

            return Template::render("trainings",
                                        TemplateContextTrainingsAndDives {
                                            name,
                                            trainings,
                                            selected_training: None,
                                            dives_for_training: vec![],
                                            successful_add: None,
                                            months,
                                        });

        }
    }

    return Template::render("index", TemplateContext { name: None });
}

#[post("/delete_dive", data = "<select_dive>")]
fn delete_dive<'a>(cookies: Cookies, select_dive: String) {

    println!("In delete dive {}", &select_dive);
    if let Some(user_id) = utils::check_if_login_cookie(cookies) {
        let name = database_api::get_name_of_logged_in_user(user_id.as_str());
       let id_ = select_dive.trim_matches('+').trim();   // This is the trainingsdive's id, be aware..
        let _result = database_api::delete_dive(id_);
    }


}

#[post("/present_selected_competition", data = "<select_competition>")]
fn present_selected_competition<'a>(cookies: Cookies,
                                    select_competition: Form<SelectCompetition>) -> Template {
    if let Some(user_id) = utils::check_if_login_cookie(cookies) {
        let name = database_api::get_name_of_logged_in_user(user_id.as_str());
        let competition: &SelectCompetition = &select_competition;

        if let Ok(competition_data) = database_api::get_competitions(user_id.as_str()) {
            let competitions = utils::get_competition_statics(competition_data);

            if let Ok(dives_for_competition) = database_api::get_dives_for_competition(competition.id) {
                let id_ = competition.id.trim_matches('+');

                return Template::render("competitions",
                                        TemplateContextCompetitionsAndDives {
                                            name,
                                            competitions,
                                            selected_competition: Some(id_.to_string()),
                                            dives_for_competition
                                        });
            }
        }
    }

    return Template::render("index", TemplateContext { name: None });
}

#[post("/present_selected_dive", data = "<select_dive>")]
fn present_selected_dive<'a>(cookies: Cookies,
                             select_dive: Form<SelectDive>) -> Template {
    if let Some(user_id) = utils::check_if_login_cookie(cookies) {
        let dive: &SelectDive = &select_dive;

        if let Ok(dives_data) = database_api::get_users_unique_dives(user_id.as_str()) {
            if let Ok(stats_for_dive) = database_api::get_stats_for_dive(&dive.id.as_str(),
                                                                         user_id.as_str()) {
                let name = database_api::get_name_of_logged_in_user(user_id.as_str());

                let dives: Vec<(String, String, String)> =
                    dives_data.into_iter()
                        .map(|f| (format!("{}", f.id),format!("{}", f.code), format!("{}", f.height)))
                        .collect();

                let (times_for_training, statistics_for_dive): (Vec<String>, Vec<String>) = stats_for_dive.into_iter().unzip();
                let id_ = dive.id.trim_matches('+');

                return Template::render("dives", TemplateContextStatisticsForDive {
                    name,
                    dives,
                    selected_dive: Some(id_.to_string()),
                    times_for_training,
                    statistics_for_dive
                });
            }
        }
    }
    return Template::render("index", TemplateContextTrainingsAndDives {
        name: None,
        trainings: vec![],
        selected_training: None,
        dives_for_training: vec![],
        successful_add: None, months:vec![]});
}

#[post("/get_log_training", data = "<get_log_training>")]
fn get_log_training<'a>(cookies: Cookies, get_log_training: Form<LogDive>) -> Template {
    if let Some(user_id) = utils::check_if_login_cookie(cookies) {
        let dive: &LogDive = &get_log_training;
        println!("Dive to add: {} h: {}", &dive.dive, &dive.height);
        let name = database_api::get_name_of_logged_in_user(user_id.as_str());
        if let Ok(()) = database_api::log_dives_on_training(user_id.as_str(), dive) {
            if let Ok(training_data) = database_api::get_trainings(user_id.as_str()) {

                let trainings = utils::get_trainings_statics(&training_data);
                let months = utils::get_month_statics(&training_data);

                return Template::render("log_training",
                                        TemplateContextTrainingsAndDives {
                                            name,
                                            trainings,
                                            selected_training: Some(dive.training_id.to_string()),
                                            dives_for_training: vec![],
                                            successful_add: Some("Added dive".to_string()),
                                            months
                                        });
            }
        }
        else {
            if let Ok(training_data) = database_api::get_trainings(user_id.as_str()) {
                let trainings = utils::get_trainings_statics(&training_data);
                            let months = utils::get_month_statics(&training_data);
                return Template::render("log_training",
                                        TemplateContextTrainingsAndDives {
                                            name,
                                            trainings,
                                            selected_training: Some(dive.training_id.to_string()),
                                            dives_for_training: vec![],
                                            successful_add: Some("Failed to add dive.".to_string()),
                                            months
                                        });
            }
        }
    }
    return Template::render("index", TemplateContext { name: None });
}


#[post("/get_log_competition", data = "<get_log_competition>")]
fn get_log_competition<'a>(cookies: Cookies,
                           get_log_competition: Form<LogCompetitionDive>) -> Template {
    if let Some(user_id) = utils::check_if_login_cookie(cookies) {
        let dive: &LogCompetitionDive = &get_log_competition;

                let name = database_api::get_name_of_logged_in_user(user_id.as_str());
        if let Ok(competition_data) = database_api::get_competitions(user_id.as_str()) {
            if let Ok(()) = database_api::log_dives_on_competition(dive) {

                let competitions = utils::get_competition_statics(competition_data);

                return Template::render("log_competition", TemplateContextCompetitions {
                    name,
                    competitions,
                    selected_competition: Some(dive.competition_id.to_string()),
                    successful_add: Some("Added dive".to_string())
                });
            } else {
                let competitions = utils::get_competition_statics(competition_data);
                return Template::render("log_competition", TemplateContextCompetitions {
                    name,
                    competitions,
                    selected_competition: Some(dive.competition_id.to_string()),
                    successful_add: Some("Failed to add dive!".to_string())
                });
            }
        }
    }
    return Template::render("index", TemplateContext { name: None });
}


#[get("/get_log_training_form")]
fn get_log_training_form(cookies: Cookies) -> Template {
    if let Some(user_id) = utils::check_if_login_cookie(cookies) {
        if let Ok(training_data) = database_api::get_trainings(user_id.as_str()) {
            let name = database_api::get_name_of_logged_in_user(user_id.as_str());

            let trainings = utils::get_trainings_statics(&training_data);
            let months = utils::get_month_statics(&training_data);

            return Template::render("log_training",
                                    TemplateContextTrainingsAndDives {
                                        name, trainings, selected_training: None,
                                        dives_for_training: vec![], successful_add: None, months });
        }
    }
    return Template::render("index", TemplateContext { name: None });
}

#[get("/get_log_competition_form")]
fn get_log_competition_form(cookies: Cookies) -> Template {
    if let Some(user_id) = utils::check_if_login_cookie(cookies) {
        if let Ok(competition_data) = database_api::get_competitions(user_id.as_str()) {
            let name = database_api::get_name_of_logged_in_user(user_id.as_str());

            let competitions = utils::get_competition_statics(competition_data);

            return Template::render("log_competition",
                                    TemplateContextCompetitions {
                                        name, competitions, selected_competition: None, successful_add: None });
        }
    }
    return Template::render("index", TemplateContext { name: None });
}

#[get("/login_page")]
pub fn login_page(cookies: Cookies) -> Template {
    println!("loginpage");
    if let Some(user_id) = utils::check_if_login_cookie(cookies) {
        let name = database_api::get_name_of_logged_in_user(user_id.as_str());
        return Template::render("index", TemplateContext { name });   // TODO whoulc redirect to logout
    }

    return Template::render("login", TemplateContext { name: None });
}

#[get("/register_page")]
pub fn register_page(mut cookies: Cookies) -> Template {
    let login_cookie: Option<Cookie> = cookies.get_private("user_id");
    if login_cookie.is_some() {
        cookies.remove_private(login_cookie.unwrap());
    }
    Template::render("register", TemplateContext { name: None })
}

#[get("/statistics/get_dives")]
pub fn get_dives(cookies: Cookies) -> Template {
    if let Some(user_id) = utils::check_if_login_cookie(cookies) {
        if let Ok(dives_data) = database_api::get_users_unique_dives(user_id.as_str()) {
            let name = database_api::get_name_of_logged_in_user(user_id.as_str());
            let dives: Vec<(String, String, String)> =
                dives_data.into_iter().map(|f|
                    (format!("{}", f.id),
                     format!("{}", f.code), format!("{}", f.height)))
                    .collect();
            return Template::render("dives", TemplateContextStatisticsForDive {
                name,
                dives,
                selected_dive: None,
                times_for_training: vec![],
                statistics_for_dive: vec![],
            });
        }
    }
    return Template::render("index", TemplateContext { name: None });
}

#[get("/get_stats_overview")]
pub fn get_stats_overview(cookies: Cookies) -> Template {
    if let Some(user_id) = utils::check_if_login_cookie(cookies) {
        if let Ok(per_group) = statistics_overview::get_stats_overview_per_group(user_id.as_str()) {
            if let Ok(per_height) = statistics_overview::get_amount_dives_per_height(user_id.as_str()) {

                let name = database_api::get_name_of_logged_in_user(user_id.as_str());


                return Template::render("dive_statistics_overview",
                                        TemplateContextStatsOverviewAllDives {
                                            name,
                                            amount_per_dive_group: per_group,
                                            amount_per_height: per_height,
                                            amount_per_dive: vec![],
                                        });

            }
        }
    }
    return Template::render("index", TemplateContext { name: None });
}

use rocket::response::content;
use std::str::FromStr;
use std::process::Command;

#[get("/get_data_for_comp_dive")]
pub fn get_data_for_comp_dive(cookies: Cookies) -> content::Json<&'static str> {
    if let Some(user_id) = utils::check_if_login_cookie(cookies) {
        if let Ok(per_comp_dive) = statistics_overview::get_amount_competition_dives(user_id.as_str()) {


            let name = database_api::get_name_of_logged_in_user(user_id.as_str());
          let serialized: String = serde_json::to_string(&per_comp_dive).unwrap();
            let x = string_to_static_str(serialized);
            let json = content::Json(x);
            return json;
        }
    }
    return content::Json("{ 'something': 'went wrong' }");
}

#[get("/get_data_for_dive")]
pub fn get_data_for_dive(cookies: Cookies) -> content::Json<&'static str> {
    if let Some(user_id) = utils::check_if_login_cookie(cookies) {
        if let Ok(per_comp_dive) = statistics_overview::get_amount_dives(user_id.as_str()) {
            let name = database_api::get_name_of_logged_in_user(user_id.as_str());
            let serialized: String = serde_json::to_string(&per_comp_dive).unwrap();

            let x = string_to_static_str(serialized);
            let json = content::Json(x);
            return json;
        }
    }
    return content::Json("{ 'something': 'went wrong' }");
}

#[get("/get_git_revision")]
pub fn get_git_revision() -> content::Plain<&'static str> {

    let  ret: String = format!("{}", env!("GIT_HASH"));

    println!("{}", &ret);
    return content::Plain(string_to_static_str(ret));
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

#[post("/present_selected_competition_dive", data = "<select_dive>")]
fn present_selected_competition_dive<'a>(cookies: Cookies,
                                         select_dive: Form<SelectDive>) -> Template {
    if let Some(user_id) = utils::check_if_login_cookie(cookies) {
        let dive: &SelectDive = &select_dive;


        if let Ok(dives_data) = database_api::get_competition_dives(user_id.as_str()) {
            let name = database_api::get_name_of_logged_in_user(user_id.as_str());

            let mut competition_dives_string: Vec<&i32> = dives_data.keys().collect();
            competition_dives_string.sort();
            let non_m = competition_dives_string;
            let competition_dives: Vec<(String, (String, Vec<(String, String)>))> =
                non_m.into_iter().map(
                    |gg|
                        {
                            let val: &(String, Vec<(String, String)>) = dives_data.get(&gg).unwrap();
//
//                            let vv: Vec<(String, String)> =
//                                val.1.iter().map(|st| (st.0.clone(), st.1.clone())).collect();
                            let struct_for_dive = (gg.to_string(), (val.0.clone(), val.1.clone()));
                            return struct_for_dive;
                        }
                ).collect();

            let id_ = dive.id.trim_matches('+');
            let i32_id = id_.parse::<i32>().unwrap();
            let x: &Vec<(String, String)> = &dives_data.borrow().get(&i32_id).unwrap().1;
            let y = x.clone();

            let rand: (Vec<String>, Vec<String>) = y.into_iter().unzip();

            let times_for_comp: Vec<String> = rand.0;
            let score_data: Vec<String> = rand.1;
            return Template::render("competition_dives", TemplateContextCompetitionDivesAndData {
                name,
                competition_dives,
                selected_comp_dive: Some(id_.to_string()),
                times_for_comp,
                score_data,
            });
        }
    }
    return Template::render("index", TemplateContext { name: None});
}

#[get("/trainings")]
fn trainings(cookies: Cookies) -> Template {
    if let Some(user_id) = utils::check_if_login_cookie(cookies) {
        if let Ok(training_data) = database_api::get_trainings(user_id.as_str()) {
            let name = database_api::get_name_of_logged_in_user(user_id.as_str());

            let trainings = utils::get_trainings_statics(&training_data);
            let months = utils::get_month_statics(&training_data);
            return Template::render("trainings",
                                    TemplateContextTrainingsAndDives {
                                        name, trainings, selected_training: None,
                                        dives_for_training: vec![] , successful_add: None, months });
        }
    }
    return Template::render("index", TemplateContext { name: None });
}

#[get("/")]
fn index(_cookies: Cookies) -> Template {
    return Template::render("index", TemplateContext { name: None });
}


#[get("/competitions")]
fn competitions(cookies: Cookies) -> Template {
    if let Some(user_id) = utils::check_if_login_cookie(cookies) {
        if let Ok(competition_data) = database_api::get_competitions(user_id.as_str()) {
            let name = database_api::get_name_of_logged_in_user(user_id.as_str());

            let competitions = utils::get_competition_statics(competition_data);

            return Template::render("competitions",
                                    TemplateContextCompetitionsAndDives {
                                        name, competitions, selected_competition: None,
                                        dives_for_competition: vec![] });
        }
    }
    return Template::render("index", TemplateContext { name: None });
}

#[get("/competition_dives")]
fn competition_dives(cookies: Cookies) -> Template {
    if let Some(user_id) = utils::check_if_login_cookie(cookies) {
        if let Ok(dives_data) = database_api::get_competition_dives(user_id.as_str()) {
            let name = database_api::get_name_of_logged_in_user(user_id.as_str());

            let mut competition_dives_string: Vec<&i32> = dives_data.keys().collect();
            competition_dives_string.sort();
            let competition_dives =
                competition_dives_string.into_iter().map(
                    |gg|
                        {
                            let val: &(String, Vec<(String, String)>) = dives_data.get(&gg).unwrap();
                            let vv: Vec<(String, String)> =
                                val.1.iter().map(|st| (st.0.clone(),
                                                       st.1.clone())).collect();
                            let struct_for_dive = (gg.to_string(), (val.0.clone(), vv));
                            return struct_for_dive;
                        }
                ).collect();

            return Template::render("competition_dives", TemplateContextCompetitionDivesAndData {
                name,
                competition_dives,
                selected_comp_dive: None,
                times_for_comp: vec![],
                score_data: vec![],
            });
        }
    }
    return Template::render("index", TemplateContext { name: None });
}

#[get("/static/<file..>")]
fn static_content(file: PathBuf) -> Result<NamedFile, Error> {
    let result: Result<NamedFile, Error> = NamedFile::open(Path::new("static/").join(file)).or_else(err);
    result
}

fn err(x: Error) -> Result<NamedFile, Error> { Err(x) }

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, get_dives, login, register,
        get_log_training_form,get_add_training, get_log_training, present_selected_training,delete_training, delete_dive,
        competitions, competition_dives, get_log_competition_form,get_add_competition,
        get_log_competition, present_selected_competition,
        present_selected_dive, present_selected_competition_dive, login_page,
        register_page, static_content, get_stats_overview, get_data_for_comp_dive, get_data_for_dive, trainings, get_git_revision])
        .attach(Template::fairing())
}


fn main() {

    rocket().launch();
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_rocket() {
        use super::rocket;
        use rocket::local::Client;
        use rocket::http::Status;
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }

}

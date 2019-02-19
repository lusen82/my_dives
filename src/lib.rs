
#![feature(custom_attribute)]
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(try_from)]
#![feature(drain_filter)]
#![feature(repeat_generic_slice)]
#![feature(try_trait)]
#![feature(plugin)]

#[macro_use] extern crate diesel;
extern crate dotenv;
#[macro_use] extern crate rocket;

extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate itertools;
extern crate core;
extern crate chrono;
extern crate tera;


pub mod schema;
pub mod old_schema;
pub mod models;
pub mod database_api;
pub mod initialize_db;
pub mod template_context;
pub mod form_data;
pub mod utils;
pub mod core_database_api;

pub mod deletion_database_api;
pub mod statistics_overview;
pub mod responders;


use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn establish_old_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL_OLD")
        .expect("DATABASE_URL_OLD must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


//fn rocket() -> rocket::Rocket {
//    rocket::ignite()
//        .mount("/", routes![index]).attach(Template::fairing())
//}
//
//#[get("/")]
//fn index(cookies: Cookies) -> Template {
//    if let Some(user_id) = utils::check_if_login_cookie(cookies) {
//        if let Ok(training_data) = database_api::get_trainings(user_id.as_str()) {
//            let name = database_api::get_name_of_logged_in_user(user_id.as_str());
//
//            let trainings = utils::get_trainings_statics(training_data);
//
//            return Template::render("index",
//                                    TemplateContextTrainingsAndDives {
//                                        name, trainings, selected_training: None,
//                                        dives_for_training: vec![] });
//        }
//    }
//    return Template::render("index", TemplateContext { name: None });
//}
#[cfg(test)]
mod tests {

    #[test]
    fn test_database_connection() {
        super::establish_connection();

    }

//
//    #[test]
//    fn test_rocket() {
//        super::establish_connection();
//        use rocket;
//        use rocket::local::Client;
//        use rocket::http::Status;
//        let client = Client::new(super::rocket()).expect("valid rocket instance");
//        let mut response = client.get("/").dispatch();
//        assert_eq!(response.status(), Status::Ok);
//        assert_eq!(response.body_string(), Some("Hello, world!".into()));
//    }

}

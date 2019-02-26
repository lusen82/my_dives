//use crate::core_database_api;
//use crate::database_api;
//use crate::utils::CliError;
//use crate::models::Training;
//use std::fs::File;
//use std::io::Write;
//
//
//pub fn produce_csv(user_id: &str) -> Result<(), CliError> {
//    println!("producing csv for user {}", &user_id);
//
//    let connection = super::establish_connection();
//
//    // Get trainings:
//
//    let trainings: Vec<Training> = database_api::get_trainings(user_id)?;
//    let mut file = File::create("foo.txt")?;
//
//    let dives_per_training = trainings.into_iter().map(|tr|
//        {
//            let training_id_id = tr.id;
//            let dives: Vec<(String, String)> = database_api::get_dives_for_training(format!("{}", training_id_id).as_str()).ok().unwrap();
//            dives.iter().for_each(|d| {
//
//                let text = format!("\"{}{}\"", d.0, d.1);
//                file.write(text.into_boxed_bytes());
//            });
//        });
//
//
//    Ok(())
//}

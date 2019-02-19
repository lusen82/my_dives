

use diesel::prelude::*;

use std::io::stdin;
use diesel;
use std::io::Lines;
use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
use std::iter;
use crate::models::NewDive;
use crate::utils::CliError;
use crate::models::LoggedInUser;
use crate::models::Diver;
use crate::models::DiversDives;
use crate::models::Dive;
use crate::models::DiversTrainings;
use crate::models::OldTraining;
use crate::models::OldCompetition;
use crate::models::OldTrainingsDives;
use crate::models::OldCompetitionDive;

pub fn create_dive_manually() -> Result<(), CliError> {
    let connection = super::establish_connection();
    println!("What dive number");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let code = &title[..(title.len() - 1)]; // Drop the newline character

    println!("What dive height");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let height = &title[..(title.len() - 1)]; // Drop the newline character

    create_dive(&connection, code, height, &0.0f32)
}

pub fn create_spring_board_dives_from_file() {
    let  path = "static/dd_table.txt";
    let all_lines: Vec<String> = read_line_by_line(path.to_string());

    let styles = vec!['A', 'B', 'C', 'D', 'A', 'B', 'C', 'D'];
    let ddives = create_dives_with_dd(all_lines, styles);
    insert_springboard_dive(ddives);
}

pub fn create_platform_training_dives_from_file() {
    let  path = "static/dd_parctice_platforms.txt";
    let all_lines: Vec<String> = read_line_by_line(path.to_string());

    let styles = vec!['A', 'B', 'C', 'D', 'A', 'B', 'C', 'D'];
    let ddives = create_dives_with_dd(all_lines, styles);
    insert_platform_training_dive(ddives);
}

pub fn create_platform_dives_from_file() -> Result<(), CliError> {
    let  path = "static/dd_table_platfrom.txt";
    let all_lines: Vec<String> = read_line_by_line(path.to_string());

    let styles = vec!['A', 'B', 'C', 'D', 'A', 'B', 'C','D','A', 'B', 'C', 'D' ];
    let ddives = create_dives_with_dd(all_lines, styles);

    insert_platform_dive(ddives)
}

pub fn restore_database() -> Result<(), CliError> {
    println!("Connecting to old db");
    // Connect to old db:
    let connection = super::establish_old_connection();

    println!("Find users.");
    use crate::schema::loggedinusers::dsl::*;
    let logged_in_users: Vec<LoggedInUser> = loggedinusers.load::<LoggedInUser>(&connection)?;


    use crate::schema::divers::dsl::*;
    let divers_res: Vec<Diver> = divers.load::<Diver>(&connection)?;
    println!("Found {} divers.", &divers_res.len());


    use crate::old_schema::trainings::dsl::*;
    let trainings_res: Vec<OldTraining> = trainings.load::<OldTraining>(&connection)?;
    println!("Found {} trainings.", &trainings_res.len());


    use crate::old_schema::competitions::dsl::*;
    let competitions_res: Vec<OldCompetition> = competitions.load::<OldCompetition>(&connection)?;
    println!("Found {} competitions.", &competitions_res.len());

    use crate::schema::dives::dsl::*;
    let dives_res: Vec<Dive> = dives.load::<Dive>(&connection)?;
    println!("Found {} dives.", dives_res.len());

    use crate::schema::diversdives::dsl::*;
    let divers_dives_res: Vec<DiversDives> = diversdives.load::<DiversDives>(&connection)?;

    use crate::schema::diverstrainings::dsl::*;
    let divers_trainings_res: Vec<DiversTrainings> = diverstrainings.load::<DiversTrainings>(&connection)?;

    use crate::old_schema::trainingsdives::dsl::*;
    let trainings_dives_res: Vec<OldTrainingsDives> = trainingsdives.load::<OldTrainingsDives>(&connection)?;

    use crate::old_schema::competitiondives::dsl::*;
    let competition_dive_res: Vec<OldCompetitionDive> = competitiondives.load::<OldCompetitionDive>(&connection)?;

    println!("All data is taken from old db.");

    // Update connection to new db

    let connection = super::establish_connection();
    println!("Inserting to new db.");
    logged_in_users.into_iter().for_each(|f|
        {
            let d : LoggedInUser = f;
            use diesel::expression::sql_literal::sql;
            let insert_into_query = format!("INSERT INTO loggedinusers (id, log_in_name, password) VALUES ({}, '{}', '{}') on conflict (id) do nothing;",
                                            d.id, d.log_in_name, d.password);
            let  _res: QueryResult<i64> = sql(insert_into_query.as_str()).get_result(&connection);

        });
    println!("Inserted users.");
    divers_res.into_iter().for_each(|f|
       {
           let d : Diver = f;
           use diesel::expression::sql_literal::sql;
           let  insert_into_query = format!("INSERT INTO divers (id, logged_in_user_id, name, born, email) VALUES ({}, {}, '{}', {}, '{}') on conflict (id) do nothing;",
                                            d.id, d.logged_in_user_id, d.name, d.born, d.email);
           let  _res: QueryResult<i64> = sql(insert_into_query.as_str()).get_result(&connection);
       });
    println!("inserted divers.");
    trainings_res.into_iter().for_each(|f|
       {
           let d : OldTraining = f;
           use diesel::expression::sql_literal::sql;
           let  insert_into_query = format!("INSERT INTO trainings (id, diver_id, date_time, feeling, comment) VALUES ({}, {}, '{}', {}, '{}') on conflict (id) do nothing;",
                                            d.id, d.diver_id, d.date_time, 0, d.comment);
           let  _res: QueryResult<i64> = sql(insert_into_query.as_str()).get_result(&connection);
       });
    println!("Inserted trainings.");
    competitions_res.into_iter().for_each(|f|
       {
           let d : OldCompetition = f;
           use diesel::expression::sql_literal::sql;
           let  insert_into_query = format!("INSERT INTO competitions  (id, diver_id, competition_name, date_time, feeling, comment) VALUES ({}, {}, '{}', '{}', {}, '{}') on conflict (id) do nothing;",
                                            d.id, d.diver_id, d.competition_name, d.date_time, 0, d.comment);
           let  _res: QueryResult<i64> = sql(insert_into_query.as_str()).get_result(&connection);
       });
    println!("Inserted competitions.");
    dives_res.into_iter().for_each(|f|
       {
           let d : Dive = f;
           use diesel::expression::sql_literal::sql;
           let  insert_into_query = format!("INSERT INTO dives  (id, dive_group, code, height, dd) VALUES ({}, {}, '{}', '{}', {}) on conflict (id) do nothing;",
                                            d.id, d.dive_group, d.code, d.height, d.dd);
           let  _res: QueryResult<i64> = sql(insert_into_query.as_str()).get_result(&connection);
       });
    println!("Inserted dives.");
    divers_dives_res.into_iter().for_each(|f|
       {
           let d : DiversDives = f;
           use diesel::expression::sql_literal::sql;
           let  insert_into_query = format!("INSERT INTO diversdives  (id, dive_id, diver_id) VALUES ({}, {}, {}) on conflict (id) do nothing;",
                                            d.id, d.dive_id, d.diver_id);
           let  _res: QueryResult<i64> = sql(insert_into_query.as_str()).get_result(&connection);
       });

    divers_trainings_res.into_iter().for_each(|f|
       {
           let d : DiversTrainings = f;
           use diesel::expression::sql_literal::sql;
           let  insert_into_query = format!("INSERT INTO diverstrainings (id, diver_id, training_id) VALUES ({}, {}, {}) on conflict (id) do nothing;",
                                            d.id, d.diver_id, d.training_id);
           let  _res: QueryResult<i64> = sql(insert_into_query.as_str()).get_result(&connection);
       });

    trainings_dives_res.into_iter().for_each(|f|
       {
           let d : OldTrainingsDives = f;
           use diesel::expression::sql_literal::sql;
           let  insert_into_query = format!("INSERT INTO trainingsdives (id, training_id, dive_id, nr_of_times, feeling, comment) VALUES ({}, {}, {}, {}, {}, '{}') on conflict (id) do nothing;",
                                            d.id, d.training_id, d.dive_id, d.nr_of_times, 0, d.comment);
           let  _res: QueryResult<i64> = sql(insert_into_query.as_str()).get_result(&connection);
       });

    competition_dive_res.into_iter().for_each(|f|
       {
           let d : OldCompetitionDive = f;
           use diesel::expression::sql_literal::sql;
                       let  insert_into_query = format!("INSERT INTO competitiondives (id, competition_id, dive_id, score, feeling, comment)  VALUES ({}, {}, {}, {}, {}, '{}') on conflict (id) do nothing;",
                                                        d.id, d.competition_id, d.dive_id, d.score, 0, d.comment);
           let  _res: QueryResult<i64> = sql(insert_into_query.as_str()).get_result(&connection);
       });
    println!("Inserted relations.");
    println!("Ready inserting, check result.");
    Ok(())

}

pub fn fix_id_sequences() -> Result<(), CliError> {

    // Update connection to new db

    let connection = super::establish_connection();
    println!("Inserting to new db.");


    let table = "trainings";
    fix_id_sequence_for_table(&connection, table);

    let table = "dives";
       fix_id_sequence_for_table(&connection, table);

    let table = "loggedinusers";
       fix_id_sequence_for_table(&connection, table);


    let table = "divers";
       fix_id_sequence_for_table(&connection, table);

    let table = "competitions";
       fix_id_sequence_for_table(&connection, table);
    let table = "diversdives";
       fix_id_sequence_for_table(&connection, table);

    let table = "trainingsdives";
       fix_id_sequence_for_table(&connection, table);

    let table = "competitiondives";
        fix_id_sequence_for_table(&connection, table);

    let table = "diverstrainings";
        fix_id_sequence_for_table(&connection, table);




    Ok(())

}

fn fix_id_sequence_for_table(connection: &PgConnection, table: &str) {

    let table_id_seq = format!("{}_id_seq", table);
    let fix_seq_query_for_table = format!("SELECT setval('{}', (SELECT MAX(id) FROM {})+1);",
                                          table_id_seq, table);
    use diesel::expression::sql_literal::sql;
    let _res: QueryResult<i64> = sql(fix_seq_query_for_table.as_str()).get_result(connection);
}
//pub fn dump_data_for_all_users() -> Result<(), CliError> {
//    let connection = super::establish_connection();
//    //let users = get_users()?;
//    //
//
//}

//fn get_users() -> Result<Vec<LoggedInUser>, CliError> {
//    use crate::schema::loggedinusers::dsl::*;
//    let users: Vec<LoggedInUsers> = loggedinusers::table.load::<LoggedInUsers>(&connection)
//        .expect("error loading users");
//    Ok(users)
//}


fn create_dives_with_dd(all_lines: Vec<String>, styles: Vec<char>) -> Vec<DDive> {
    let mut ddives: Vec<DDive> = vec![];
    for line in all_lines {
        let split1: Vec<&str> = line.split_whitespace().collect();

        let mut split = split1.into_iter();
      //  println!("{:?}", &split);
        if let Some(next) = split.next() {

            let instruction: String = next.to_string();
            let mut col = 0;
            if let Ok(dive_code) = instruction.parse::<i64>() {

                let mut dds_per_style: Vec<f32> = iter::repeat(0.0f32).take(styles.len()).collect();

                while let Some(sp) = split.next() {
                    if &sp.len() > &1 {
                        if let Ok(dd) = sp.parse::<f32>() {
                            dds_per_style[col] = dd;
                            col = col + 1;
                        }
                    }
                    else if "-".eq(sp) {
                        dds_per_style[col] = 0.0;
                        col = col + 1;
                    }
                }

                let code_styles: Vec<String> = iter::repeat(format!("{}", dive_code)).take(styles.len()).collect();
                let zipped_code_styles = code_styles.iter().zip(styles.iter()).map(|(x, y)| {
                    let mut s = x.clone();
                    s.push(*y);
                    s
                }).collect();
                let non_mut = dds_per_style;
                ddives.push(DDive { code: zipped_code_styles, dd: non_mut });
            }
        }
    }
    ddives
}

fn insert_springboard_dive(ddives: Vec<DDive>) {
    let connection = super::establish_connection();

    for dv in ddives {
        for i in 0..8 {
            let height = match i {
                 4 | 5 | 6 | 7=> "3m",
                 _ => "1m"
            };
            let post = create_dive(&connection, &dv.code[i], &height, &dv.dd[i]);
            if post.is_err() {
                println!("Failed creating dive.")
            }

        }
    }
}

fn insert_platform_training_dive(ddives: Vec<DDive>) {
    let connection = super::establish_connection();

    for dv in ddives {
        for i in 0..8 {
            let height = match i {
                 4 | 5 | 6 | 7=> "1m Platform",
                 _ => "3m Platform"
            };
            let post = create_dive(&connection, &dv.code[i], &height, &dv.dd[i]);
            if post.is_err() {
                println!("Failed creating dive.")
            }
        }
    }
}

fn insert_platform_dive(ddives: Vec<DDive>) -> Result<(), CliError> {
    let connection = super::establish_connection();

    for dv in ddives {
        for i in 0..dv.code.len() {
            let height = match i {
                 0 | 1 | 2 | 3 => "10m Platform",
                 4 | 5 | 6 | 7 => "7m Platform",
                _ => "5m Platform"
            };
            create_dive(&connection, &dv.code[i], &height, &dv.dd[i])?;
        }
    }
    Ok(())

}

fn create_dive<'a>(conn: &PgConnection, ccode: &'a str, hheight: &'a str, ddd: &'a f32) -> Result<(), CliError> {

    if  ddd > &0.1 {
        let ddive_group: &i16 = &get_group_for_dive(ccode.as_ref());

        use crate::schema::dives;
        let new_dive = NewDive {
            code: ccode,
            dive_group: ddive_group,
            height: hheight,
            dd: ddd
        };
        use crate::schema::dives::dsl::*;
        let  existing_dive : Vec<Dive> = dives.filter(code.eq(ccode)).filter(dive_group.eq(ddive_group)).filter(height.eq(hheight)).load::<Dive>(conn)?;

        if existing_dive.is_empty() {

        println!("Inserting dive {} on height {} with dd {}", ccode, hheight, ddd);
        let bool_expression_methods = diesel::insert_into(dives::table).values(&new_dive).execute(conn);
        if bool_expression_methods.is_err() {
            println!("Failed creating dive.")
        }
        }
    }
    Ok(())

}

fn get_group_for_dive(code: &str) -> i16 {
    let first_char = code.chars().into_iter().next().unwrap();
    let group = match first_char {
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => {
            match code.chars().into_iter().next().unwrap() {
                '1' => 51,
                _ => 52
            }
        },
        '6' => {
            match code.chars().into_iter().next().unwrap() {
                '1' => 61,
                _ => 62
            }
        }
        _ => 0
    };
    group
}

struct DDive
{
    code: Vec<String>,
    dd: Vec<f32>
}

pub fn read_line_by_line(file_name: String) -> Vec<String> {

    let f = File::open(file_name).expect("file not found");

    let file: BufReader<&File> = BufReader::new(&f);
    let lines_read: Lines<BufReader<&File>> = file.lines();

    let mut all_lines_vec: Vec<String> = Vec::new();
    for line in lines_read
        {
            let ll = line.unwrap();
            all_lines_vec.push(ll);
        }

    return all_lines_vec;

}



#[cfg(test)]
mod tests {

//    #[test]
//    fn test_database() {
//        super::create_dive_manually();
//    }

    #[test]
    fn test_parse_and_insert_dives_from_file() {
        println!("Starting test.");
        //super::restore_database();
       // super::fix_id_sequences();

        //super::create_spring_board_dives_from_file();
     //   super::create_platform_dives_from_file();
        super::create_platform_training_dives_from_file();
    }
//
//    #[test]
//    fn test_dump_current_data() {
//        super::dump_data_for_all_users();
//    }

}


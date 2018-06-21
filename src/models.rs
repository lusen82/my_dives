use super::schema::diversdives;
use super::schema::divers;
use super::schema::loggedinusers;
use super::schema::dives;
use super::schema::trainings;
use super::schema::diverstrainings;
use super::schema::trainingsdives;
use super::schema::competitiondives;

use super::schema::competitions;


#[derive(Queryable, Identifiable, Associations, PartialEq)]
#[table_name="dives"]
pub struct Dive {
    pub id: i32,
    pub dive_group: i16,
    pub code: String,
    pub height: String,
    pub dd: f32
}

#[derive(Queryable, Identifiable, Associations, PartialEq)]
#[table_name="trainings"]
pub struct Training {
    pub id: i32,
    pub diver_id: i32,
    pub date_time: String,     // Ska bytas till date type.
    pub feeling: i16,
    pub comment: String
}

#[derive(Queryable, Identifiable, Associations, PartialEq)]
#[table_name="trainings"]
pub struct OldTraining {
    pub id: i32,
    pub diver_id: i32,
    pub date_time: String,     // Ska bytas till date type.
    pub comment: String
}

#[derive(Identifiable, Queryable, PartialEq, Associations)]
#[belongs_to(LoggedInUser)]
#[table_name="divers"]
pub struct Diver {
    pub id: i32,
    pub logged_in_user_id: i32,
    pub name: String,
    pub born: i32,
    pub email: String,
}


#[derive(Queryable, Identifiable, Associations, PartialEq)]
#[belongs_to(Diver)]
#[table_name="competitions"]
pub struct Competition {
    pub id: i32,
    pub diver_id: i32,
    pub competition_name: String,
    pub date_time: String,     // Ska bytas till date type.
    pub feeling: i16,
    pub comment: String,
}


#[derive(Queryable, Identifiable, Associations, PartialEq)]
#[belongs_to(Diver)]
#[table_name="competitions"]
pub struct OldCompetition {
    pub id: i32,
    pub diver_id: i32,
    pub competition_name: String,
    pub date_time: String,     // Ska bytas till date type.
    pub comment: String,
}

#[derive(Queryable, Identifiable, Associations, PartialEq)]
#[belongs_to(Competition, Dive)]
#[table_name="competitiondives"]
pub struct CompetitionDive {
    pub id: i32,
    pub competition_id: i32,
    pub dive_id: i32,
    pub score: f32,
    pub feeling: i16,
    pub comment: String,
}

#[derive(Queryable, Identifiable, Associations, PartialEq)]
#[belongs_to(Competition, Dive)]
#[table_name="competitiondives"]
pub struct OldCompetitionDive {
    pub id: i32,
    pub competition_id: i32,
    pub dive_id: i32,
    pub score: f32,
    pub comment: String,
}

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[belongs_to(Diver, Dive)]
#[table_name="diversdives"]
pub struct DiversDives {
    pub id: i32,
    pub dive_id: i32,
    pub diver_id: i32,
}

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[belongs_to(Training, Dive)]
#[table_name="trainingsdives"]
pub struct TrainingsDives {
    pub id: i32,
    pub training_id: i32,
    pub dive_id: i32,
    pub nr_of_times: i16,
    pub feeling: i16,
    pub comment: String,

}

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[belongs_to(Training, Dive)]
#[table_name="trainingsdives"]
pub struct OldTrainingsDives {
    pub id: i32,
    pub training_id: i32,
    pub dive_id: i32,
    pub nr_of_times: i16,
    pub comment: String,

}


#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[belongs_to(Diver, Training)]
#[table_name="diverstrainings"]
pub struct DiversTrainings {
    pub id: i32,
    pub diver_id: i32,
    pub training_id: i32
}

#[derive(Queryable, PartialEq, Identifiable)]
#[table_name="loggedinusers"]
pub struct LoggedInUser {
    pub id: i32,
    pub log_in_name: String,
    pub password: String
}


#[derive(Insertable)]
#[table_name="dives"]
pub struct NewDive<'a> {
    pub code: &'a str,
    pub dive_group: &'a i16,
    pub height: &'a str,
    pub dd: &'a f32
}


#[derive(Insertable)]
#[table_name="trainings"]
pub struct NewTraining<'a> {
    pub diver_id: &'a i32,
    pub date_time: &'a str,
    pub feeling: &'a i16,
    pub comment: &'a str
}

#[derive(Insertable)]
#[table_name="divers"]
pub struct NewDiver<'a> {
    pub logged_in_user_id: &'a i32,
    pub name:  &'a str,
    pub born:  &'a i32,
    pub email: &'a str
}


#[derive(Insertable)]
#[table_name="loggedinusers"]
pub struct NewLoggedInUser<'a> {
    pub log_in_name: &'a str,
    pub password: &'a str
}

#[derive(Insertable)]
#[table_name="competitiondives"]
pub struct NewCompetitionDive<'a> {
    pub dive_id: &'a i32,
    pub competition_id: &'a i32,
    pub score: &'a f32,
    pub feeling: &'a i16,
    pub comment: &'a str,
}

#[derive(Insertable)]
#[table_name="competitions"]
pub struct NewCompetition<'a> {
    pub diver_id: &'a i32,
    pub competition_name: &'a str,
    pub date_time: &'a str,     // Ska bytas till date type.

    pub feeling: &'a i16,
    pub comment: &'a str,
}


#[derive(Insertable)]
#[table_name="diversdives"]
pub struct NewDiversDive<'a> {
    pub diver_id: &'a i32,
    pub dive_id: &'a i32,
}

#[derive(Insertable)]
#[table_name="trainingsdives"]
pub struct NewTrainingsDive<'a> {
    pub training_id: &'a i32,
    pub dive_id: &'a i32,
    pub nr_of_times: &'a i16,
    pub feeling: &'a i16,
    pub comment: &'a str,
}


#[derive(Insertable)]
#[table_name="diverstrainings"]
pub struct NewDiversTraining<'a> {
    pub diver_id: &'a i32,
    pub training_id: &'a i32,
}


use rocket::http::RawStr;

#[derive(FromForm)]
pub struct UserLogin<'r> {
    pub username: &'r RawStr,
    pub password: &'r RawStr,
    pub remember_me: bool
}

#[derive(FromForm)]
pub struct UserRegister<'r> {
    pub name: &'r RawStr,
    pub username: &'r RawStr,
    pub password: &'r RawStr,
    pub born: &'r RawStr,
    pub email: &'r RawStr,
}

#[derive(FromForm)]
pub struct LogDive<'r> {
    pub training_id: &'r RawStr,
    pub dive: &'r RawStr,
    pub height: &'r RawStr,
    pub number: &'r RawStr,
    pub feeling: &'r RawStr,
    pub comment: &'r RawStr,

}

#[derive(FromForm)]
pub struct LogCompetitionDive<'r> {
    pub competition_id: &'r RawStr,
    pub dive: &'r RawStr,
    pub height: &'r RawStr,
    pub score: &'r RawStr,
    pub feeling: &'r RawStr,
    pub comment: &'r RawStr,

}

#[derive(FromForm)]
pub struct AddTraining<'r> {
    pub date: &'r RawStr,
    pub feeling: &'r RawStr,
    pub comment: &'r RawStr,
}

#[derive(FromForm)]
pub struct AddCompetition<'r> {
    pub date: &'r RawStr,
    pub competition_name: &'r RawStr,
    pub feeling: &'r RawStr,
    pub comment: &'r RawStr,
}

#[derive(FromForm)]
pub struct SelectCompetition<'r> {
    pub id: &'r RawStr,
}

#[derive(FromForm)]
pub struct SelectTraining<'r> {
    pub id: &'r RawStr,
}

#[derive(FromForm)]
pub struct SelectDive<'r> {
    pub id: &'r RawStr,
}

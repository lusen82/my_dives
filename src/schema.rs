table! {
    competitiondives (id) {
        id -> Int4,
        competition_id -> Int4,
        dive_id -> Int4,
        score -> Float4,
        feeling -> Int2,
        comment -> Text,
    }
}

table! {
    competitions (id) {
        id -> Int4,
        diver_id -> Int4,
        competition_name -> Text,
        date_time -> Text,
        feeling -> Int2,
        comment -> Text,
    }
}

table! {
    divers (id) {
        id -> Int4,
        logged_in_user_id -> Int4,
        name -> Text,
        born -> Int4,
        email -> Text,
    }
}

table! {
    diversdives (id) {
        id -> Int4,
        dive_id -> Int4,
        diver_id -> Int4,
    }
}

table! {
    diverstrainings (id) {
        id -> Int4,
        diver_id -> Int4,
        training_id -> Int4,
    }
}

table! {
    dives (id) {
        id -> Int4,
        dive_group -> Int2,
        code -> Text,
        height -> Text,
        dd -> Float4,
    }
}

table! {
    loggedinusers (id) {
        id -> Int4,
        log_in_name -> Varchar,
        password -> Varchar,
    }
}

table! {
    trainings (id) {
        id -> Int4,
        diver_id -> Int4,
        date_time -> Text,
        feeling -> Int2,
        comment -> Text,
    }
}

table! {
    trainingsdives (id) {
        id -> Int4,
        training_id -> Int4,
        dive_id -> Int4,
        nr_of_times -> Int2,
        feeling -> Int2,
        comment -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    competitiondives,
    competitions,
    divers,
    diversdives,
    diverstrainings,
    dives,
    loggedinusers,
    trainings,
    trainingsdives,
);

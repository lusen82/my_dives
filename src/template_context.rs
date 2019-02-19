

#[derive(Serialize)]
pub struct TemplateContext {
    pub name: Option<String>
}

#[derive(Serialize)]
pub struct TemplateContextStatsOverviewAllDives {
    pub name: Option<String>,
    pub amount_per_dive_group: Vec<String>,
    pub amount_per_height: Vec<String>,
    pub amount_per_dive: Vec<Vec<String>>
}

#[derive(Serialize)]
pub struct TemplateContextCompetitions {
    pub name: Option<String>,
    pub competitions: Vec<Vec<String>>,
    pub selected_competition: Option<String>,
    pub successful_add: Option<String>
}

#[derive(Serialize)]
pub struct TemplateContextCompetitionsAndDives {
    pub name: Option<String>,
    pub competitions: Vec<Vec<String>>,
    pub selected_competition: Option<String>,
    pub dives_for_competition: Vec<String>
}

#[derive(Serialize)]
pub struct TemplateContextCompetitionDivesAndData {
    pub name: Option<String>,
    pub competition_dives: Vec<(String, (String, Vec<(String, String)>))>,
    pub selected_comp_dive: Option<String>,
    pub times_for_comp: Vec<String>,
    pub score_data: Vec<String>
}

#[derive(Serialize)]
pub struct TemplateContextTrainingsAndDives {
    pub name: Option<String>,
    pub trainings: Vec<(String, String)>,
    pub selected_training: Option<String>,
    pub dives_for_training: Vec<(String, String)>,
    pub successful_add: Option<String>
}

#[derive(Serialize)]
pub struct TemplateContextStatisticsForDive {
    pub name: Option<String>,
    pub dives: Vec<(String, String, String)>,
    pub selected_dive: Option<String>,
    pub times_for_training: Vec<String>,
    pub statistics_for_dive: Vec<String>
}



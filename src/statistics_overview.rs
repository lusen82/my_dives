

use core_database_api;
use utils;
use models::Dive;
use utils::CliError;

pub fn get_stats_overview_per_group(user_id: &str) -> Result<Vec<String>, CliError> {

    let all_users_dives = core_database_api::get_all_users_dives(user_id)?;

    return Ok(get_amount_dives_per_group(all_users_dives));
}

fn get_amount_dives_per_group(dives: Vec<Dive>) -> Vec<String> {
    let dive_groups : Vec<i16> = dives.into_iter().map(|u| u.dive_group).collect();

    let forward: &Vec<&i16> = &dive_groups.iter().filter(|p| *p == &get_i16("1")).collect();
    let back: &Vec<&i16>  = &dive_groups.iter().filter(|p| *p == &get_i16("2")).collect();
    let reverse: &Vec<&i16>  = &dive_groups.iter().filter(|p| *p == &get_i16("3")).collect();
    let inward: &Vec<&i16>  = &dive_groups.iter().filter(|p| *p == &get_i16("4")).collect();
    let twist: &Vec<&i16>  = &dive_groups.iter().filter(|p| *p == &get_i16("51") || *p == &get_i16("52")).collect();

    let v: Vec<String> = vec!(format!("{}", forward.len()), format!("{}", &back.len()),
                              format!("{}", &reverse.len()), format!("{}", &inward.len()),
                              format!("{}", &twist.len()));

    let arr = v.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    let statics: Vec<String> = arr.into_iter().map(|f| f.to_string()).collect();
     statics
    //(statics[0], statics[1], statics[2], statics[3], statics[4])
}

fn get_i16(x: &str) -> i16 {
    return x.parse::<i16>().unwrap();
}


pub fn get_amount_dives_per_height(user_id: &str) -> Result<Vec<String>, CliError> {
    let dives: Vec<Dive> = core_database_api::get_all_users_unique_dives(user_id)?;

    let dive_heights : Vec<String> = dives.into_iter().map(|u| u.height).collect();
    let one: &Vec<&String>  = &dive_heights.iter().filter(|p| "1m".to_string().eq(*p)).collect();
    let three: &Vec<&String>  = &dive_heights.iter().filter(|p| "3m".to_string().eq(*p)).collect();
    let platform: &Vec<&String>  = &dive_heights.iter().filter(|p| "5m Platform".to_string().eq(*p)).collect();
    let  ret = vec![one.len().to_string(), three.len().to_string(), platform.len().to_string()];
     Ok(ret)
//    Ok((utils::to_static_str_from_i32(one.len() as i32), utils::to_static_str_from_i32(three.len() as i32),
//        utils::to_static_str_from_i32(platform.len() as i32)))
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_get_amount_dives_per_group() {
        //super::get_amount_dives_per_group("1");
    }


}

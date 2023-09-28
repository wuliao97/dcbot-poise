use poise::ChoiceParameter;
use crate::utils::constant::*;


#[derive(ChoiceParameter, Debug)]
pub enum SpotifySearchType {
    #[name = "Track"]
    A,
    #[name = "Artist"]
    B,
    #[name = "Album"]
    C,
}

#[derive(ChoiceParameter, Debug)]
pub enum SpotifySearchLanguage {
    #[name = "EN - English"]
    A,
    #[name = "JP - Japan"]
    B,
}




pub fn spotify_err_msg_one(local: Option<&str>) -> &str {
    return match local {
        Some(lang) => match lang {
            "ja" => SPTFY_ERR_MSG_CASE_ONE_JA,
            _ => SPTFY_ERR_MSG_CASE_ONE
        },
        _ => SPTFY_ERR_MSG_CASE_ONE,
    }
}



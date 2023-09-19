use crate::utils::constant::*;

// pub fn check_option_lang(language: Option<&str>) {
//
// }

pub fn spotify_err_msg_one(local: Option<&str>) -> &str {
    return match local {
        Some(lang) => match lang {
            "ja" => SPTFY_ERR_MSG_CASE_ONE_JA,
            _ => SPTFY_ERR_MSG_CASE_ONE
        },
        _ => SPTFY_ERR_MSG_CASE_ONE,
    }
}
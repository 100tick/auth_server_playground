use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize)]
pub enum Gender {
    Man,
    Woman,
    Unknown,
}

impl Gender {
    pub fn as_str(&self) -> &'static str {
        match self {
            Gender::Man => "m",
            Gender::Woman => "w",
            Gender::Unknown => "_",
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Gender::Man => "m",
            Gender::Woman => "w",
            Gender::Unknown => "_",
        }
        .to_string()
    }
}

#[derive(Serialize, Deserialize)]
pub enum Locale {
    En,
    Kr,
}

// impl From<Locale> for String {
//     fn from(l: Locale) -> String {
//         match l {
//             Locale::Kr => "kr",
//             Locale::En => "en",
//         }
//         .to_string()
//     }
// }
// impl From<String> for Locale {
//     fn from(s: String) -> Self {
//         match s.as_str() {
//             "kr" => Self::Kr,
//             "en" => Self::En,
//         }
//     }
// }

impl Locale {
    pub fn as_str(&self) -> &'static str {
        match self {
            Locale::En => "en",
            Locale::Kr => "kr",
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Locale::En => "en",
            Locale::Kr => "kr",
        }
        .to_string()
    }
}

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i64,
    pub google_id: String,
    pub avatar_url: String,
    pub email: String,
    pub name: String,
    pub locale: String,
    // user_info
    pub first_name: String,
    pub last_name: String,
    pub gender: Option<String>,
    pub phone: Option<String>,
    pub birth_date: Option<NaiveDate>,
    //
    // pub created_at: NaiveDateTime,
    // pub updated_at: NaiveDateTime,
}

#[derive(Debug, FromRow)]

pub struct UserInfo {
    pub user_id: i64,
    pub first_name: String,
    pub last_name: String,
    pub gender: Option<String>,
    pub phone: Option<String>,
    pub birth_date: Option<NaiveDate>,
}

#[derive(Deserialize, Serialize)]
pub struct NewUser {
    pub google_id: String,
    pub avatar_url: String,
    pub email: String,
    pub name: String,
    pub locale: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateUser {
    // pub avatar_url: String,
    // pub email: String,
    pub name: Option<String>,
    pub locale: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<Option<String>>,
    pub phone: Option<Option<String>>,
    pub birth_date: Option<Option<NaiveDate>>,
}

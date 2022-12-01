use sea_orm::entity::prelude::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum Gender {
    #[sea_orm(string_value = "man")]
    Man,
    #[sea_orm(string_value = "woman")]
    Woman,
    #[sea_orm(string_value = "private")]
    Private,
}

// use chrono::{NaiveDate, Utc};
// use entity::prelude::{TempUser, User};
// use entity::{temp_user, user};
// use sea_orm::prelude::Uuid;
// use sea_orm::{
//     ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect,
// };
// use serde::{Deserialize, Serialize};
// use thiserror::Error;

// use crate::app_error::AppResult;

// pub struct UserRepo {
//     conn: DatabaseConnection,
// }

// #[derive(Error, Debug, Clone, PartialEq, Eq)]
// pub enum SignUpError {
//     #[error("duplicated user")]
//     UserDuplicated(String),
//     #[error("it's timeout need re-auth")]
//     SignUpTimeout(String),
// }

// #[derive(Serialize, Deserialize)]
// pub struct NewUser {
//     pub first_name: String,
//     pub last_name: String,
//     pub gender: String,
//     pub birth_date: NaiveDate,
// }

// impl UserRepo {
//     pub fn new(conn: DatabaseConnection) -> Self {
//         Self { conn }
//     }

//     pub async fn is_exists_by_google_id(&self, google_id: &str) -> AppResult<bool> {
//         let found_user_id = User::find()
//             .filter(temp_user::Column::Id.eq(google_id))
//             .select_only()
//             .column(temp_user::Column::Id)
//             .one(&self.conn)
//             .await?;

//         Ok(found_user_id.is_some())
//     }

//     pub async fn find_user_by_google_id(&self, google_id: &str) -> AppResult<Option<user::Model>> {
//         let found_user = User::find()
//             .filter(temp_user::Column::GoogleId.eq(google_id))
//             .one(&self.conn)
//             .await?;

//         Ok(found_user)
//     }

//     pub async fn create_and_get_user(&self, new_user: user::ActiveModel) -> AppResult<user::Model> {
//         let created_user = User::insert(new_user).exec(&self.conn).await?;

//         Ok(created_user)
//     }

//     pub async fn find_temp_user_by_google_id(
//         &self,
//         google_id: &str,
//     ) -> AppResult<Option<temp_user::Model>> {
//         let found_temp_user = TempUser::find()
//             .filter(user::Column::GoogleId.eq(google_id))
//             .one(&self.conn)
//             .await?;

//         Ok(found_temp_user)
//     }

//     pub async fn create_temp_user(&self, new_temp_user: temp_user::ActiveModel) -> AppResult<()> {
//         // expires_in -> 10mins
//         // if temp_user exists, overwrite it
//         // if not exists, create it

//         let google_id = new_temp_user.google_id.as_ref();
//         let is_exists_user = self.is_exists_by_google_id(google_id).await?;
//         if is_exists_user {
//             return Err(SignUpError::UserDuplicated("".to_string()).into());
//         }

//         let found_temp_user = self.find_temp_user_by_google_id(google_id).await?;
//         if found_temp_user.is_some() {
//             self.delete_temp_user_by_id(found_temp_user.unwrap().id)
//                 .await?;
//         }

//         let new_temp_user: temp_user::ActiveModel = new_temp_user.into();
//         new_temp_user.save(&self.conn).await?;
//         Ok(())
//     }

//     pub async fn create_user(
//         &self,
//         google_id: String,
//         new_user: user::NewModel,
//     ) -> AppResult<user::Model> {
//         // 1. check if user exists
//         // 2. fetch temp_user
//         // 3. check if temp_user is expired
//         // 4-1. if expired, request to re-authenticate
//         // 4-2. if not expired, create user
//         let user_exists = self.is_exists_by_google_id(&google_id).await?;

//         if user_exists {
//             return Err(SignUpError::UserDuplicated("".to_string()).into());
//         }

//         let found_temp_user = match self.find_temp_user_by_google_id(&google_id).await? {
//             Some(found_temp_user) => found_temp_user,
//             None => {
//                 return Err(SignUpError::SignUpTimeout("".to_string()).into());
//             }
//         };

//         if is_temp_user_expired(&found_temp_user) {
//             return Err(SignUpError::SignUpTimeout("".to_string()).into());
//         }

//         let new_user = new_user.into_with_temp_user(found_temp_user);
//         let created_user = new_user.insert(&self.conn).await?;

//         Ok(created_user)
//     }

//     pub fn update() {}

//     pub async fn delete_temp_user_by_id(&self, id: Uuid) -> AppResult<()> {
//         TempUser::delete_by_id(id).exec(&self.conn).await?;
//         Ok(())
//     }
// }

// fn is_temp_user_expired(&temp_user: &temp_user::Model) -> bool {
//     let now = Utc::now();
//     let expires_at = temp_user.expires_at;

//     if now > expires_at {
//         return true;
//     }
//     false
// }

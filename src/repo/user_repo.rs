use sqlx::PgPool;

use crate::error::AppResult;

use crate::model::{NewUser, UpdateUser, User};

#[derive(Clone)]
pub struct UserRepo {
    pool: PgPool,
}

impl UserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user_and_get_id(
        &self,
        NewUser {
            google_id,
            avatar_url,
            email,
            name,
            locale,
            first_name,
            last_name,
        }: &NewUser,
    ) -> AppResult<i64> {
        let mut tx = self.pool.begin().await?;

        let user_id = sqlx::query_scalar(
            r#"
INSERT INTO "user"
    ( google_id, avatar_url, email, name, locale )
    VALUES ( $1, $2, $3, $4, $5 )
    RETURNING id;
        "#,
        )
        .bind(google_id)
        .bind(avatar_url)
        .bind(email)
        .bind(name)
        .bind(locale)
        .fetch_one(&mut tx)
        .await?;

        sqlx::query(
            r#"
INSERT INTO "user_info"
    ( user_id, first_name, last_name )
    VALUES ( $1, $2, $3 );
        "#,
        )
        .bind(user_id)
        .bind(first_name)
        .bind(last_name)
        .execute(&mut tx)
        .await?;

        tx.commit().await?;

        Ok(user_id)
    }

    pub async fn is_user_exists_by_google_id(&self, google_id: &str) -> AppResult<bool> {
        let is_user_exists = sqlx::query_scalar(
            r#"
SELECT EXISTS (
    SELECT FROM "user"
    WHERE "google_id" = $1 
);
"#,
        )
        .bind(google_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(is_user_exists)
    }

    pub async fn find_user_by_google_id(&self, google_id: &str) -> AppResult<Option<User>> {
        let user = sqlx::query_as(
            r#"
SELECT * FROM "user_with_info"
WHERE google_id = $1;
            "#,
        )
        .bind(google_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn update_user(
        &self,
        id: i64,
        UpdateUser {
            name,
            locale,
            first_name,
            last_name,
            gender,
            phone,
            birth_date,
        }: &UpdateUser,
    ) -> AppResult<()> {
        let mut tx = self.pool.begin().await?;

        sqlx::query(
            r#"
UPDATE "user" SET
    name = COALESCE($2, name),
    locale = COALESCE($3, locale)
WHERE id = $1;
"#,
        )
        .bind(id)
        .bind(name)
        .bind(locale)
        .execute(&mut tx)
        .await?;

        sqlx::query(
            r#"
UPDATE "user_info" SET
    first_name = COALESCE($2, first_name),
    last_name = COALESCE($3, last_name),
    phone = CASE WHEN $4 IS TRUE THEN $5 ELSE phone END,
    gender = CASE WHEN $6 IS TRUE THEN $7 ELSE gender END,
    birth_date = CASE WHEN $8 IS TRUE THEN $9 ELSE birth_date END
WHERE user_id = $1;
        "#,
        )
        .bind(id)
        .bind(first_name)
        .bind(last_name)
        .bind(phone.is_some())
        .bind(phone)
        .bind(gender.is_some())
        .bind(gender)
        .bind(birth_date.is_some())
        .bind(birth_date)
        .execute(&mut tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }
}

#[cfg(test)]
mod user_repo_tests {
    use super::UserRepo;
    use crate::{
        db::connect_to_db,
        model::{Gender, NewUser, UpdateUser},
    };
    use chrono::NaiveDate;
    use dotenv::dotenv;
    use std::env;

    async fn new_user_repo() -> UserRepo {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").unwrap();
        println!("{db_url}");
        let pool = connect_to_db(&db_url).await;
        UserRepo::new(pool)
    }

    #[tokio::test]
    pub async fn test_create_user_and_get_id() {
        let user_repo = new_user_repo().await;
        let new_user = NewUser {
            google_id: "123456789123456789000".to_string(),
            email: "aa".to_string(),
            avatar_url: "a".to_string(),
            first_name: "a".to_string(),
            last_name: "a".to_string(),
            name: "a".to_string(),
            locale: "aa".to_string(),
        };

        let user_id = user_repo.create_user_and_get_id(&new_user).await.unwrap();

        println!("{user_id}");
        // user_repo
    }

    #[tokio::test]
    async fn test_is_user_exists_by_google_id() {
        let user_repo = new_user_repo().await;
        let google_id = "123456789123456789000";
        let user1_exists = user_repo
            .is_user_exists_by_google_id(google_id)
            .await
            .unwrap();

        let google_id2 = "12345678912345678900";
        let user2_exists = user_repo
            .is_user_exists_by_google_id(google_id2)
            .await
            .unwrap();

        println!("user1: {user1_exists}");
        println!("user2: {user2_exists}");
    }

    #[tokio::test]
    async fn test_find_user_by_google_id() {
        let user_repo = new_user_repo().await;
        let google_id = "123456789123456789000";
        let user1 = user_repo.find_user_by_google_id(google_id).await.unwrap();

        let google_id2 = "12345678912345678900";
        let user2 = user_repo.find_user_by_google_id(google_id2).await.unwrap();

        println!("user1: {user1:?}");
        println!("user2: {user2:?}");
    }

    #[tokio::test]
    async fn test_update_user() {
        let user_repo = new_user_repo().await;

        let update_user = UpdateUser {
            // name: Some("xxxx".to_string()),
            name: None,
            first_name: Some("z".to_string()),
            last_name: Some("z".to_string()),
            birth_date: Some(Some(NaiveDate::from_ymd(2022, 10, 1))),
            // phone: Some(Some("1".to_string())),
            phone: Some(None),
            // phone: None,
            gender: Some(Some(Gender::Man.to_string())),
            locale: Some("xx".to_string()),
        };

        user_repo.update_user(1, &update_user).await.unwrap();

        // let google_id2 = "12345678912345678900";
        // let user2 = user_repo.find_user_by_google_id(google_id2).await.unwrap();

        // println!("user1: {user1:?}");
    }
}

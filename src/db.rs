use sqlx::PgPool;

// TODO: NEED TO HANDLE ERROR CASES
// pub fn connect_to_db(db_url: &str) -> PgPool {
//     let manager = ConnectionManager::<PgConnection>::new(db_url);

//     let pool = Pool::new(manager).unwrap();

//     pool.get().unwrap()
// }

// pub fn get_connection_pool(db_url: &str) -> Pool<ConnectionManager<PgConnection>> {
//     let manager = ConnectionManager::<PgConnection>::new(db_url);

//     Pool::builder()
//         .test_on_check_out(true)
//         .build(manager)
//         .expect("Could not build connection pool")
// }

pub async fn connect_to_db(url: &str) -> PgPool {
    PgPool::connect(url)
        .await
        .expect("Database connection failed")
}

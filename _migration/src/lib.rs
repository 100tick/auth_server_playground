pub use sea_orm_migration::*;

mod m20220706_001937_init;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220706_001937_init::Migration)]
    }
}

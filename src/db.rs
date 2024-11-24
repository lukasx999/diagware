use sqlx::SqlitePool;
use tokio::runtime::Runtime as TokioRuntime;
use std::future::Future;

pub mod model;
use model::{Module, TargetValue};


// Modify here!
const DB_FILENAME: &str = "src/database.db";


#[derive(Debug)]
pub struct DB {
    conn: SqlitePool,
    rt:   TokioRuntime,
}

impl DB {

    pub fn new() -> Result<Self, sqlx::Error> {
        let rt = TokioRuntime::new()?;
        Ok(Self {
            conn: rt.block_on(SqlitePool::connect(DB_FILENAME))?,
            rt
        })
    }

    fn run_sync<F>(&self, future: F) -> F::Output
        where F: Future
    {
        self.rt.block_on(future)
    }

    pub fn get_modules_all(&self) -> Result<Vec<Module>, sqlx::Error> {

        let future =
        sqlx::query_as!(Module, "SELECT * FROM modules")
            .fetch_all(&self.conn);

        Ok(self.run_sync(future)?)

    }

    pub fn get_module_by_id(
        &self, id: i64
    ) -> Result<Module, sqlx::Error> {

        let future =
        sqlx::query_as!(Module, "SELECT * FROM modules WHERE id=?1", id)
            .fetch_one(&self.conn);

        Ok(self.run_sync(future)?)

    }

    pub fn get_module_by_serial(
        &self, serial: &str
    ) -> Result<Module, sqlx::Error> {

        let future =
        sqlx::query_as!(Module, "SELECT * FROM modules WHERE serial=?1", serial)
            .fetch_one(&self.conn);

        Ok(self.run_sync(future)?)

    }

    pub fn module_add(&self, module: Module) -> Result<(), sqlx::Error> {

        let future =
        sqlx::query!("INSERT INTO modules (id, name, serial) VALUES (?1, ?2, ?3)",
            module.id,
            module.name,
            module.serial)
            .execute(&self.conn);

        self.run_sync(future)?;
        Ok(())

    }

    pub fn module_update_by_id(
        &self,
        id: i64,
        module: Module
    ) -> Result<(), sqlx::Error> {

        assert!(module.id == None);

        let future =
        sqlx::query!("UPDATE modules SET name=?1, serial=?2 WHERE id=?3;",
            module.name,
            module.serial,
            id)
            .execute(&self.conn);

        self.run_sync(future)?;
        Ok(())

    }

    pub fn module_delete_by_id(
        &self,
        id: i64
    ) -> Result<(), sqlx::Error> {

        let future =
        sqlx::query!("DELETE FROM modules WHERE id=?1", id)
            .execute(&self.conn);

        self.run_sync(future)?;
        Ok(())

    }

    pub async fn get_targetvalues_all(
        &self
    ) -> Result<Vec<TargetValue>, sqlx::Error> {

        Ok(sqlx::query_as!(TargetValue, "SELECT * FROM targetvalues")
            .fetch_all(&self.conn)
            .await?)

    }

}

use sqlx::SqlitePool;
use tokio::runtime::Runtime as TokioRuntime;

pub mod model;
use model::{Module, TargetValue};



// Modify here!
const DB_FILENAME: &str = "src/database.db";




#[derive(Debug)]
pub struct DB {
    conn: SqlitePool,
    rt: TokioRuntime,
}

impl DB {

    pub fn new() -> Result<Self, sqlx::Error> {
        let rt = TokioRuntime::new()?;

        Ok(Self {
            conn: rt.block_on(SqlitePool::connect(DB_FILENAME))?,
            rt,
        })
    }

    pub fn get_modules_all(&self) -> Result<Vec<Module>, sqlx::Error> {

        Ok(self.rt.block_on(
            sqlx::query_as!(Module, "SELECT * FROM modules")
                .fetch_all(&self.conn))?)

    }

    pub async fn get_module_by_id(
        &self, id: i64
    ) -> Result<Module, sqlx::Error> {

        Ok(sqlx::query_as!(Module, "SELECT * FROM modules WHERE id=?1", id)
            .fetch_one(&self.conn)
            .await?)

    }

    pub async fn get_module_by_serial(
        &self, serial: &str
    ) -> Result<Module, sqlx::Error> {

        Ok(sqlx::query_as!(Module, "SELECT * FROM modules WHERE serial=?1", serial)
            .fetch_one(&self.conn)
            .await?)

    }

    pub async fn module_add(&self, module: Module) -> Result<(), sqlx::Error> {

        sqlx::query!("INSERT INTO modules (id, name, serial) VALUES (?1, ?2, ?3)",
            module.id,
            module.name,
            module.serial)
            .execute(&self.conn)
            .await?;

        Ok(())

    }

    pub async fn module_delete_by_id(
        &self, id: i64
    ) -> Result<(), sqlx::Error> {

        sqlx::query!("DELETE FROM modules WHERE id=?1", id)
            .execute(&self.conn)
            .await?;

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

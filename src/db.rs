use sqlx::SqlitePool;
use tokio::runtime::Runtime as TokioRuntime;
use std::future::Future;

pub mod model;
use model::{Module, TargetValue, Matrix};


// Modify here!
const DB_FILENAME: &str = "src/database.db";


#[derive(Debug)]
pub struct DB {
    conn: SqlitePool,
    rt:   TokioRuntime,
}

#[allow(dead_code)]
impl DB {

    pub fn new() -> sqlx::Result<Self> {
        let rt = TokioRuntime::new()?;
        Ok(Self {
            conn: rt.block_on(SqlitePool::connect(DB_FILENAME))?,
            rt
        })
    }

    fn run_sync<F>(&self, fut: F) -> F::Output
where F: Future
    {
        self.rt.block_on(fut)
    }

    pub fn get_modules_all(&self) -> sqlx::Result<Vec<Module>> {
        let fut = sqlx::query_as!(
            Module,
            "SELECT * FROM modules"
        ).fetch_all(&self.conn);

        Ok(self.run_sync(fut)?)
    }

    pub fn get_module_by_id(&self, id: i64) -> sqlx::Result<Module> {
        let fut = sqlx::query_as!(
            Module,
            "SELECT * FROM modules WHERE id=?1",
            id
        ).fetch_one(&self.conn);

        Ok(self.run_sync(fut)?)
    }

    pub fn get_module_by_serial(&self, serial: &str) -> sqlx::Result<Module> {
        let fut = sqlx::query_as!(
            Module,
            "SELECT * FROM modules WHERE serial=?1",
            serial
        ).fetch_one(&self.conn);

        Ok(self.run_sync(fut)?)

    }

    pub fn module_add(&self, module: Module) -> sqlx::Result<()> {
        let fut = sqlx::query!(
            "INSERT INTO modules (id, name, serial) VALUES (?1, ?2, ?3)",
            module.id,
            module.name,
            module.serial
        ).execute(&self.conn);

        self.run_sync(fut)?;
        Ok(())
    }

    pub fn module_update_by_id(
        &self,
        module: Module
    ) -> sqlx::Result<()> {
        let fut = sqlx::query!(
            "UPDATE modules SET name=?1, serial=?2 WHERE id=?3;",
            module.name,
            module.serial,
            module.id
        ).execute(&self.conn);

        self.run_sync(fut)?;
        Ok(())
    }

    pub fn module_delete_by_id(&self, id: i64) -> sqlx::Result<()> {
        let fut = sqlx::query!(
            "DELETE FROM modules WHERE id=?1",
            id
        ).execute(&self.conn);

        self.run_sync(fut)?;
        Ok(())
    }

    pub fn get_targetvalues_all(&self) -> sqlx::Result<Vec<TargetValue>> {
        let fut = sqlx::query_as!(
            TargetValue,
            "SELECT * FROM targetvalues"
        ).fetch_all(&self.conn);

        Ok(self.run_sync(fut)?)
    }

    pub fn get_targetvalue_by_id(&self, id: i64) -> sqlx::Result<Vec<TargetValue>> {
        let fut = sqlx::query_as!(
            TargetValue,
            "SELECT * FROM targetvalues WHERE module_id=?1",
            id
        ).fetch_all(&self.conn);

        Ok(self.run_sync(fut)?)
    }

    pub fn get_matrix_by_id(&self, id: i64) -> sqlx::Result<Matrix> {
        // NOTE: problem: Sqlite3 only supports 64bit signed integer types
        // hence we are reading in 64bit ints, then converting to 16bit uints

        let fut = sqlx::query!(
            "SELECT * FROM matrix WHERE id=?1",
            id
        ).fetch_one(&self.conn);

        // raw is a anonymous struct from sqlx
        let raw = self.run_sync(fut)?;

        Ok(Matrix::new(
            raw.id,
            raw.gnd      as u16,
            raw.v_plus   as u16,
            raw.v_minus  as u16,
            raw.dds_out1 as u16,
            raw.dds_out2 as u16,
            raw.dds_out3 as u16,
            raw.adc_in1  as u16,
            raw.adc_in2  as u16
        ))
    }





}

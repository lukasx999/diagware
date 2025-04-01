use sqlx::SqlitePool;
use tokio::runtime::Runtime as TokioRuntime;
use std::future::Future;

pub mod model;
use model::{Module, TargetValue, Matrix, Document};



#[derive(Debug)]
pub struct DB {
    conn: SqlitePool,
    rt:   TokioRuntime,
}

impl DB {

    pub fn new() -> sqlx::Result<Self> {
        let rt = TokioRuntime::new()?;
        Ok(Self {
            conn: rt.block_on(SqlitePool::connect(env!("DATABASE_URL")))?,
            rt
        })
    }

    fn run_sync<F: Future>(&self, fut: F) -> F::Output {
        self.rt.block_on(fut)
    }

    pub fn get_modules_all(&self) -> sqlx::Result<Vec<Module>> {
        let fut = sqlx::query_as!(
            Module,
            "SELECT * FROM modules"
        ).fetch_all(&self.conn);

        self.run_sync(fut)
    }

    pub fn get_module_by_id(&self, id: i64) -> sqlx::Result<Module> {
        assert_ne!(id, 0);

        let fut = sqlx::query_as!(
            Module,
            "SELECT * FROM modules WHERE id=?1",
            id
        ).fetch_one(&self.conn);

        self.run_sync(fut)
    }

    pub fn get_module_by_serial(&self, serial: &str) -> sqlx::Result<Module> {
        let fut = sqlx::query_as!(
            Module,
            "SELECT * FROM modules WHERE serial=?1",
            serial
        ).fetch_one(&self.conn);

        self.run_sync(fut)

    }

    pub fn get_targetvalues_by_id(&self, id: i64) -> sqlx::Result<Vec<TargetValue>> {
        let fut = sqlx::query_as!(
            TargetValue,
            "SELECT * FROM targetvalues WHERE module_id=?1",
            id
        ).fetch_all(&self.conn);

        self.run_sync(fut)
    }

    pub fn get_matrix_by_id(&self, id: i64) -> sqlx::Result<Matrix> {
        // Sqlite3 only supports 64bit signed integer types
        // hence we are reading in 64bit ints, then converting to 16bit uints
        // therefore we can't use sqlx::query_as!

        let fut = sqlx::query!(
            "SELECT * FROM matrix WHERE id=?1",
            id
        ).fetch_one(&self.conn);

        // raw is a anonymous struct from sqlx
        let raw = self.run_sync(fut)?;

        Ok(Matrix::new(
            raw.id,
            raw.module_id,
            raw.gnd           as u16,
            raw.v_plus        as u16,
            raw.v_minus       as u16,
            raw.dds_out_plus  as u16,
            raw.dds_out_minus as u16,
            raw.v3_3          as u16,
            raw.adc_in1       as u16,
            raw.adc_in2       as u16
        ))
    }

    pub fn get_documents_by_id(&self, id: i64) -> sqlx::Result<Vec<Document>> {
        let fut = sqlx::query_as!(
            Document,
            "SELECT * FROM documents WHERE module_id=?1",
            id
        ).fetch_all(&self.conn);

        self.run_sync(fut)
    }

}

use sqlx::SqlitePool;


// Modify here!
const DB_FILENAME: &str = "src/database.db";


// Model

#[derive(Debug, Clone)]
pub struct Module {
    pub id: Option<i64>,
    pub name: String,
    pub serial: String,
}

impl Module {
    pub fn new(id: Option<i64>, name: &str, serial: &str) -> Self {
        Self {
            id,
            name:   name.to_owned(),
            serial: serial.to_owned()
        }
    }
}






#[derive(Debug)]
pub struct DB {
    conn: SqlitePool,
}

impl DB {

    pub async fn new() -> Result<Self, sqlx::Error> {
        Ok(Self {
            conn: SqlitePool::connect(DB_FILENAME).await?
        })
    }

    pub async fn get_modules_all(&self) -> Result<Vec<Module>, sqlx::Error> {

        Ok(sqlx::query_as!(Module, "SELECT * FROM modules")
            .fetch_all(&self.conn)
            .await?)

    }

    pub async fn get_module_by_id(
        &self, id: i64
    ) -> Result<Module, sqlx::Error> {

        Ok(sqlx::query_as!(Module, "SELECT * FROM modules WHERE id=?1", id)
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

}

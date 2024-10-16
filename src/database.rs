use mysql::prelude::*;
use mysql::*;

pub fn create_pool(url: &str) -> Result<Pool, Error> {
    Pool::new(url)
}

// 起動時に一度だけ実行するクエリ
pub fn single_execute_query<T: Queryable>(conn: &mut T) -> Result<(), Error> {
    conn.exec_drop("DROP TABLE IF EXISTS load_test_table", ())?;

    conn.exec_drop(
        "CREATE TABLE IF NOT EXISTS load_test_table (
            id INT PRIMARY KEY AUTO_INCREMENT,
            column1 VARCHAR(255),
            column2 INT
        )",
        (),
    )?;

    Ok(())
}

// 繰り返し実行されるクエリ
pub fn execute_query<T: Queryable>(conn: &mut T) -> Result<(), Error> {
    // let _: Vec<Row> = conn.query("SELECT * FROM load_test_table LIMIT 10")?;

    conn.exec_drop(
        "INSERT INTO load_test_table (column1, column2) VALUES (?, ?)",
        ("value1", 30),
    )?;

    Ok(())
}
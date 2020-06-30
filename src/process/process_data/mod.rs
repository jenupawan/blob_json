use super::odbc::*;
use std::env;
use std::fs::File;
use std::io::Write;
use super::writer::write_to_file;
pub fn connect(output_file: &File) -> std::result::Result<(), odbc::DiagnosticRecord> {
    let env = create_environment_v3().map_err(|e| e.unwrap())?;

    let buffer = env::var("connection_string").expect("connection string not found in env");
    let conn = env.connect_with_connection_string(&buffer)?;
    execute_statement(output_file, &conn)
}
pub fn execute_statement<'env>(
     output_file: &File,
    conn: &Connection<'env>,
) -> odbc::Result<()> {
    let stmt = Statement::with_parent(conn)?;

    let sql_text = env::var("query").expect("query not found env");
    println!("executing the sql statement ");
    match stmt.exec_direct(&sql_text)? {
        Data(mut stmt) => {
            let cols = stmt.num_result_cols()?;
            while let Some(mut cursor) = stmt.fetch()? {
                for i in 1..(cols + 1) {
                    match cursor.get_data::<&str>(i as u16)? {
                        Some(val) => write_to_file(output_file, val),
                        None => print!(" NULL"),
                    }
                }
            }
        }
        NoData(_) => println!("Query executed, no data returned"),
    }
    Ok(())
}

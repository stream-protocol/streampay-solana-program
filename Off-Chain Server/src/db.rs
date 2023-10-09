// db.rs

use rusqlite::{Connection, Result};

fn initialize_db() -> Result<()> {
    let conn = Connection::open("stream_payments.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS stream_payments (
            id INTEGER PRIMARY KEY,
            recipient TEXT NOT NULL,
            start_time INTEGER,
            interval INTEGER,
            payment_amount INTEGER,
            remaining_balance INTEGER
         )",
        [],
    )?;
    Ok(())
}

use mysql::prelude::*;
use mysql::*;
use std::{env, io, thread, time};

pub fn _createdb(
    database_name: &str,
    username: &str,
    password: &str,
    hostname: &str,
    port: u16,
) -> Result<(), mysql::Error> {
    let max_attempts = 10;

    for i in 0..max_attempts {
        println!("DOLLY WAKE UP ({}/{})", i + 1, max_attempts);

        match _connect_and_create(database_name, username, password, hostname, port) {
            Ok(_) => return Ok(()),
            Err(err) => {
                println!("Attempt {}: {:?}", i + 1, err);
            }
        }

        if i != max_attempts - 1 {
            thread::sleep(time::Duration::from_secs(5));
        } else {
            println!("Max attempts have been reached");
        }
    }

    Err(Error::from(mysql::Error::from(io::Error::new(
        io::ErrorKind::Other,
        "Max attempts reached",
    ))))
}

fn _connect_and_create(
    database_name: &str,
    username: &str,
    password: &str,
    hostname: &str,
    port: u16,
) -> Result<(), mysql::Error> {
    let opts = Opts::from_url(&format!(
        "mysql://{}:{}@{}:{}/",
        username, password, hostname, port
    ))?;

    let pool = Pool::new(opts)?;

    let mut conn = pool.get_conn()?;

    conn.query_drop(&format!("CREATE DATABASE IF NOT EXISTS {}", database_name))?;
    conn.query_drop(&format!("USE {}", database_name))?;

    conn.query_drop(
        r#"
        CREATE TABLE IF NOT EXISTS social_credits (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user varchar(255) NOT NULL,
            credits INT,
            job varchar(255),
            salary INT
        );

        CREATE TABLE IF NOT EXISTS silly_messages (
            id INT NOT NULL PRIMARY KEY AUTO_INCREMENT,
            text TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS morning_messages (
            id INT NOT NULL PRIMARY KEY AUTO_INCREMENT,
            message TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS ai_dolly (
            id INT NOT NULL PRIMARY KEY AUTO_INCREMENT,
            context LONGTEXT NOT NULL
        );
    "#,
    )?;

    Ok(())
}

fn _establish_connection() -> Result<Pool, mysql::Error> {
    let database_name = "dolly_parton";
    let username = env::var("SQL_USERNAME").expect("Expected a SQL_USERNAME in the environment");
    let password = env::var("SQL_PASSWORD").expect("Expected a SQL_PASSWORD in the environment");
    let hostname = env::var("HOSTNAME").expect("Expected a HOSTNAME in the environment");
    let port = 3306;

    let opts = Opts::from_url(&format!(
        "mysql://{}:{}@{}:{}/{}",
        username, password, hostname, port, database_name
    ))?;

    Ok(Pool::new(opts)?)
}

pub fn _putindb(user: &str, credits: u16) -> Result<(), mysql::Error> {
    let pool = _establish_connection()?;
    let mut conn = pool.get_conn()?;

    let stmt = conn.prep("INSERT INTO social_credits (user, credits) VALUES (?, ?)")?;
    conn.exec_drop(&stmt, (user, credits))?;

    Ok(())
}

pub fn _add_silly_message(message: &str) -> Result<(), mysql::Error> {
    println!("Adding {} to silly_messages", message);
    let pool = _establish_connection()?;
    let mut conn = pool.get_conn()?;

    let stmt = conn.prep("INSERT INTO silly_messages (text) VALUES (?)")?;
    conn.exec_drop(&stmt, (message,))?;

    Ok(())
}

pub fn _add_context_to_dolly_ai(context: String) -> Result<(), mysql::Error> {
    let pool = _establish_connection()?;
    let mut conn = pool.get_conn()?;

    let stmt = conn.prep("INSERT INTO ai_dolly (context) VALUES (?)")?;
    conn.exec_drop(&stmt, (context,))?;

    Ok(())
}

pub fn _add_goodmorning_message(message: &str) -> Result<(), mysql::Error> {
    println!("Adding {} to goodmorning_messages", message);
    let pool = _establish_connection()?;
    let mut conn = pool.get_conn()?;

    let stmt = conn.prep("INSERT INTO morning_messages (message) VALUES (?)")?;
    conn.exec_drop(&stmt, (message,))?;

    Ok(())
}

pub fn _getuserinfo(user: &str) -> Result<Option<(String, i32)>, mysql::Error> {
    let pool = _establish_connection()?;
    let mut conn = pool.get_conn()?;

    let stmt = conn.prep("SELECT user, credits FROM social_credits WHERE user = ?")?;
    let mut rows = conn.exec_iter(&stmt, (user,))?;

    if let Some(row) = rows.next() {
        let (username, credits) = from_row::<(String, i32)>(row?);
        Ok(Some((username, credits)))
    } else {
        Ok(None)
    }
}

pub fn _get_dolly_context() -> Result<Vec<i32>, mysql::Error> {
    let pool = _establish_connection()?;
    let mut conn = pool.get_conn()?;
    let mut found_contexts: Vec<i32> = Vec::new();

    let rows =
        conn.query_iter("SELECT context FROM ai_dolly WHERE id = (SELECT MAX(id) FROM ai_dolly);")?;

    for row in rows {
        let row = row?;
        let context_string: String = from_row(row);

        let no_backets_context: String = context_string.replace("[", "").replace("]", "");

        for ctx in no_backets_context.split(", ") {
            let converted_to_i32: i32 = ctx.parse().unwrap();
            found_contexts.push(converted_to_i32);
        }
    }

    Ok(found_contexts)
}

pub fn _get_random_silly_message() -> Result<Option<(String, i32)>, mysql::Error> {
    let pool = _establish_connection()?;
    let mut conn = pool.get_conn()?;

    let query = "SELECT text, id FROM silly_messages ORDER BY RAND() LIMIT 1";
    let mut rows = conn.query_iter(query)?;

    if let Some(row) = rows.next() {
        let (message, id): (String, i32) = mysql::from_row(row?);
        Ok(Some((message, id)))
    } else {
        Ok(None)
    }
}

pub fn _get_random_good_morning_message() -> Result<Option<(String, i32)>, mysql::Error> {
    let pool = _establish_connection()?;
    let mut conn = pool.get_conn()?;

    let query = "SELECT message, id FROM morning_messages ORDER BY RAND() LIMIT 1";
    let mut rows = conn.query_iter(query)?;

    if let Some(row) = rows.next() {
        let (message, id): (String, i32) = mysql::from_row(row?);
        Ok(Some((message, id)))
    } else {
        Ok(None)
    }
}

pub fn _add_credits(user: &str, new_credits: i32) -> Result<Option<(String, i32)>, mysql::Error> {
    let pool = _establish_connection()?;
    let mut conn = pool.get_conn()?;

    // Step 1: Get the current credits for the user
    let select_stmt = conn.prep("SELECT user, credits FROM social_credits WHERE user = ?")?;
    let mut select_rows = conn.exec_iter(&select_stmt, (user,))?;

    if let Some(row) = select_rows.next() {
        let (username, current_credits) = from_row::<(String, i32)>(row?);

        // Release the mutable borrow on `conn` by dropping select_rows
        drop(select_rows);

        // Step 2: Calculate the new total credits
        let total_credits = current_credits + new_credits;

        // Step 3: Update the database with the new total credits
        let update_stmt = conn.prep("UPDATE social_credits SET credits = ? WHERE user = ?")?;
        conn.exec_drop(&update_stmt, (total_credits, user))?;

        Ok(Some((username, total_credits)))
    } else {
        // User not found, return None
        Ok(None)
    }
}

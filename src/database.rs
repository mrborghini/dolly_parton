use mysql::prelude::*;
use mysql::*;
use std::env;

fn establish_connection() -> Result<Pool, mysql::Error> {
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

pub fn putindb(user: &str, credits: u16) -> Result<(), mysql::Error> {
    let pool = establish_connection()?;
    let mut conn = pool.get_conn()?;

    let stmt = conn.prep("INSERT INTO social_credits (user, credits) VALUES (?, ?)")?;
    conn.exec_drop(&stmt, (user, credits))?;

    Ok(())
}

pub fn getuserinfo(user: &str) -> Result<Option<(String, i32)>, mysql::Error> {
    let pool = establish_connection()?;
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

pub fn add_credits(user: &str, credits: i32) -> Result<Option<(String, i32)>, mysql::Error> {
    let pool = establish_connection()?;
    let mut conn = pool.get_conn()?;

    let stmt = conn.prep("UPDATE social_credits SET credits = ? WHERE user = ?")?;
    let mut rows = conn.exec_iter(&stmt, (credits, user,))?;

    if let Some(row) = rows.next() {
        let (username, credits) = from_row::<(String, i32)>(row?);
        Ok(Some((username, credits)))
    } else {
        Ok(None)
    }
}

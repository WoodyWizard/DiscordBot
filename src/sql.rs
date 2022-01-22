use mysql::*;
use mysql::prelude::*;
use serenity::{
    model::{channel::Message},
};





#[derive(Debug, PartialEq, Eq)]
struct Words<'a> {
    word: Box<&'a String>,
    username: &'a String,
    count: i32,
}



pub fn sql_sett() -> Result<PooledConn> {
    let url = Opts::from_url("mysql://root:root@localhost:3306/my_database")?;
    let conn = mysql::Pool::new(url)?.get_conn()?;
    Ok(conn)
}




pub fn sql_handler(msg: &Message) {
        let mut _conn = sql_sett().unwrap();
        let vecinput = vec![Words{word: Box::new(&msg.content), username: &msg.author.name, count: 0}]; 
        let mut localword = match *vecinput[0].word {
            i => &*i,
        };
        let end_w = localword.len();        
        _conn.exec_batch(
            r"INSERT INTO words (word, username, count)
            VALUES (:word, :username, :count)",
            vecinput.iter().map(| p | params! {
                "word" => &localword[0..end_w],
                "username" => &p.username,
                "count" => p.count,
            }));
}










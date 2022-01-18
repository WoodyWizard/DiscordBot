use std::env::{self, temp_dir};
use rand::Rng;
use std::thread;
use std::time::Duration;
use mysql::*;
use mysql::prelude::*;




use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, guild::GuildStatus, id::{ChannelId, GuildId},},
    prelude::*,
    utils::{MessageBuilder, Colour},
};

#[derive(Debug, PartialEq, Eq)]
struct Words<'a> {
    word: Box<&'a String>,
    username: &'a String,
    count: i32,
}


struct Handler;


#[async_trait]
impl EventHandler for Handler {


    async fn message(&self, context: Context, msg: Message) {
        let mut accept_v: bool = false;


        let mut _conn = sql_sett().unwrap();

        let vecinput = vec![Words{word: Box::new(&msg.content), username: &msg.author.name, count: 0}]; 
        let mut localword = match *vecinput[0].word {
            i => &*i,
        };

        println!("localword : {} ", localword);
        let end_w = localword.len();
        println!("end_w : {}", end_w);

        _conn.exec_batch(
            r"INSERT INTO words (word, username, count)
            VALUES (:word, :username, :count)",
            vecinput.iter().map(| p | params! {
                "word" => &localword[0..end_w],
                "username" => &p.username,
                "count" => p.count,
            }));


        if msg.author.id.0 == 152858153719955457 {
            if msg.content == "!ping" {
                let channel = match msg.channel_id.to_channel(&context).await {
                    Ok(channel) => channel,
                    Err(why) => {
                        println!("Error getting channel: {:?}", why);

                        return;
                    },
                };

                // The message builder allows for creating a message by
                // mentioning users dynamically, pushing "safe" versions of
                // content (such as bolding normalized content), displaying
                // emojis, and more.
                let response = MessageBuilder::new()
                    .push("User ")
                    .push_bold_safe(&msg.author.name)
                    .push(" used the 'ping' command in the ")
                    .mention(&channel)
                    .push(" channel")
                    .build();

                if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                    println!("Error sending message: {:?}", why);
                }
            }
            if msg.content == "!dnd" {
                context.dnd().await;
            }
            if msg.content == "!online" {
                context.online().await;
            }
            if msg.content == "!guildid" {
                if let Some(ggid) = msg.guild_id {
                    println!("{}", ggid.0);
                }            
            }
            if msg.content == "!roleids" {
                if let Some(ggid) = msg.guild_id {
                     println!("Role Id's Method Call");
                     let local_hashmap = ggid.roles(&context.http);
                     for i in local_hashmap.await.iter() {
                      // println!("{:?}", i);
                       for x in i.values() {
                            println!("{}", x);
                            println!("x name = {}", x.name);
                       }
                     }
                }                
            }

            if &msg.content[0..4] == "!rgb" { // Currenlty unavailable
               match msg.guild_id {
                    Some(x) => {
                        if x.0 == 152858221277609984 {
                           // println!("759795278269251637 ready | role mentioned: {}", msg.mention_roles[0].0);
                            let mention_role = x.roles(&context.http).await.unwrap();
                            let role_for_rgb = mention_role.get(&msg.mention_roles[0]).unwrap();
                            let mut color1: u64 = 0;
                            loop {
                                if color1 > 16777215 {color1 = 0;}
                                color1 = color1 + 49344;
                                
                                thread::sleep(Duration::from_secs(2)); 
                               
                                if let Err(why) = role_for_rgb.edit(&context.http,|mut r|{
                                           r.colour(color1/50);
                                           r
                                        }).await
                                    {
                                    println!("Error Edit Colour {}", why);
                                    }
                            }
                        }
                    },
                    None => println!("This error, it is not from 759795278269251637"),
                } 
            }

            if msg.content == "!thisroleid" {
                let response = MessageBuilder::new()
                    .push("Roleid: ")
                    .push_bold_safe(&msg.mention_roles[0].0)
                    .build();
                if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                    println!("Error sending message: {:?}", why);
                }
            }
     }
    // PUBLIC COMMANDS
 
            if msg.content == "!accept" { // For different actions from users
               accept_v = true; 
            
            }

            if msg.content == "!roll" { // Not ready at full
              accept_v = false; 
                let mut randomgenerate: u8;
                {
                    let mut rng = rand::thread_rng();
                    randomgenerate = rng.gen_range(0..100); 
                }
               
                if msg.mentions.len() == 1 {
                    println!("Mentioned user {}", msg.mentions[0].name);
                     let response = MessageBuilder::new()
                        .push("User ")
                        .push_bold_safe(&msg.author.name)
                        .push(" Send Duel Request to: ")
                        .push_bold_line_safe(&msg.mentions[0].name)
                        .push(" and rolled the number: ")
                        .push_bold_safe(randomgenerate)
                        .push(" To accept duel requst, type !accept ")
                        .build();

                     if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                        println!("Error sending message: {:?}", why);
                     }
                     while accept_v == false {
                             if accept_v == true {
                                {
                                     let mut rng = rand::thread_rng();
                                     randomgenerate = rng.gen_range(0..100);   
                                }
                                let response = MessageBuilder::new()
                                    .push("User ")
                                    .push_bold_safe(&msg.author.name)
                                    .push(" Accept the request and rolled the number :  ")
                                    .push_bold_safe(randomgenerate)
                                    .build();

                                if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                                    println!("Error sending message: {:?}", why);
                                }
                 
                               accept_v = false; 
                             }
                      }
                }
                else { 
                    let response = MessageBuilder::new()
                        .push("User ")
                        .push_bold_safe(&msg.author.name)
                        .push(" Rolled the number:  ")
                        .push_bold_safe(randomgenerate)
                        .build();

                    if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }
 

}

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected! | id: {} | guild_id: {}", ready.user.name, ready.user.id.0, ready.guilds[0].id().0);
        match &ready.guilds[0] {
            GuildStatus::OnlineGuild(x) => println!(" name {}", x.name),
            GuildStatus::OnlinePartialGuild(x) => println!(" name {}", x.name),
            _ => println!("NotOnlineGuild"),
        }
    }
    
}
    

fn sql_sett() -> Result<PooledConn> {


let url = Opts::from_url("mysql://root:root@localhost:3306/my_database")?;
    
let pool = mysql::Pool::new(url)?;

let conn = pool.get_conn()?;

return Ok(conn);
}



#[tokio::main]
async fn main() {


// Configure the client with your Discord bot token in the environment.
    let token = "Mzk2NDI3NzYxMDc5ODEyMTA2.WkbAHw.qmGKpRaWN2-GPpJ_NSHrRnn_7F8";
    let mut client =
        Client::builder(&token).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
        };

}

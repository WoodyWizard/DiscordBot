use crate::lib::get_first_word;
use rand::Rng;
use std::ops::{DerefMut, Deref};
use std::{thread, borrow::Borrow};
use std::time::Duration;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, guild::GuildStatus, id::{ChannelId, GuildId},},
    prelude::*,
    utils::{MessageBuilder, Colour},
};
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

pub struct CommHandler {
accept: Arc<Mutex<bool>>,

}

impl CommHandler {

    pub fn new() -> CommHandler {
        CommHandler{accept: Arc::new(Mutex::new(false))}
    }

    pub async fn change_accept(&self, value: bool) { *self.accept.lock().unwrap() = value;  }

    fn get_accept(&self) -> bool {
        *self.accept.lock().unwrap()
    }

    pub async fn roll_function(&self, context: &Context, msg: &Message) {
            if get_first_word(&msg.content)  == "!roll" { // Not ready at full
                    self.change_accept(false).await; 
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
                         while self.get_accept() == false && msg.author.id.0 == msg.mentions[0].id.0 {
                                if self.get_accept() == true {
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
                     
                                    self.change_accept(false).await;
                                    break;
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

}


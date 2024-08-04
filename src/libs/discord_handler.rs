use crate::logic::{calc_mastery, calc_rank};
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

pub struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!") {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content.starts_with("mastery:") {
            let sn: Vec<&str> = msg.content.split(":").collect();
            let tag = "jp1"; // tag取得の実装
            if sn.len() >= 2 {
                match calc_mastery::resp_mastery(sn[1], tag) {
                    Ok(data_vec) => {
                        let data = data_vec.join("\n");
                        if let Err(why) = msg.channel_id.say(&ctx.http, data) {
                            println!("Error sending message: {:?}", why);
                        }
                    },
                    Err(err) => {
                        println!("{}", err);
                    }
                }
            }
        } else if  msg.content.starts_with("rate:") {
            let sn: Vec<&str> = msg.content.split(":").collect();
            let tag = "jp1"; // tag取得の実装
            if sn.len() >= 2 {
                match calc_rank::resp_league(sn[1], tag) {
                    Ok(data_vec) => {
                        let data = data_vec.join("\n");
                        if let Err(why) = msg.channel_id.say(&ctx.http, data) {
                            println!("Error sending message: {:?}", why);
                        }
                    },
                    Err(err) => {
                        println!("{}", err);
                    }
                }
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
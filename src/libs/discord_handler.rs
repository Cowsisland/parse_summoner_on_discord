// use serenity::{
//     model::{channel::Message, gateway::Ready},
//     prelude::*,
// };

// struct Handler;

// impl EventHandler for Handler {
//     fn message(&self, ctx: Context, msg: Message) {
//         if msg.content == "!ping" {
//             if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!") {
//                 println!("Error sending message: {:?}", why);
//             }
//         } else if msg.content.starts_with("mastery:") {
//             let sn: Vec<&str> = msg.content.split(":").collect();
//             if sn.len() >= 2 {
//                 match resp_mastery(sn[1]) {
//                     Ok(data_vec) => {
//                         let data = data_vec.join("\n");
//                         if let Err(why) = msg.channel_id.say(&ctx.http, data) {
//                             println!("Error sending message: {:?}", why);
//                         }
//                     },
//                     Err(err) => {
//                         println!("{}", err);
//                     }
//                 }
//             }
//         } else if  msg.content.starts_with("rate:") {
//             let sn: Vec<&str> = msg.content.split(":").collect();
//             if sn.len() >= 2 {
//                 match resp_league(sn[1]) {
//                     Ok(data_vec) => {
//                         let data = data_vec.join("\n");
//                         if let Err(why) = msg.channel_id.say(&ctx.http, data) {
//                             println!("Error sending message: {:?}", why);
//                         }
//                     },
//                     Err(err) => {
//                         println!("{}", err);
//                     }
//                 }
//             }
//         }
//     }

//     fn ready(&self, _: Context, ready: Ready) {
//         println!("{} is connected!", ready.user.name);
//     }
// }
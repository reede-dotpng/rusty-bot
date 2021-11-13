use serenity::async_trait;
use serenity::client::{EventHandler, Context};
use serenity::model::prelude::{Message, Ready};

use super::ConfigStore;
use crate::commands::BOT_COMMANDS;


pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, new_message: Message) {
        let msgs_config = &ctx.data.read().await.get::<ConfigStore>().unwrap().clone();
        let mut contains_command = false;
        
        for command_name in BOT_COMMANDS.iter() {
            if new_message.content.contains(command_name) {
                contains_command = true
            }
        }
        
        if contains_command && msgs_config.read().await.get_clear_calls() {
            new_message.delete(ctx.http).await.unwrap();
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Logged in as {}", ready.user.name);
    }
}
#![warn(clippy::pedantic)]

use serenity::{prelude::{GatewayIntents, EventHandler, Context}, Client, model::prelude::{Ready, Message}};
use uwuifier::uwuify_str_sse;

const UWU_STRING: Option<&str> = option_env!("UWU_STRING");
const FALLBACK_UWU: &str = "!UWU";

#[tokio::main]
async fn main() {
	// Read token
	dotenv::dotenv().ok();
	let token = std::env::var("DISCORD_TOKEN").expect("discord token");

	// Prepare intents
	let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

	// Client init
	let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("create client");

	// Run client
	if let Err(why) = client.start().await {
		eprintln!("Bot crash: {}", why);
	}
}

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
	async fn ready(&self, _: Context, r: Ready) {
		eprintln!("Connected as {}", r.user.name);
	}

	async fn message(&self, c: Context, m: Message) {
		if let Err(e) = m.channel_id.start_typing(&c.http) {
			eprintln!("Failed to start typing because {}", e);
		}
		// Stop if the author is another bot.
		if m.author.bot {
			return;
		}

		// Stop if the channel is not uwu
		if !m.channel(&c.http).await.map_or(false, |channel| {
			channel.guild().map_or(false, |channel| {
				channel.topic.map_or(false, |topic| topic.contains(UWU_STRING.unwrap_or(FALLBACK_UWU)))
			})
		}) {
			return
		}
		
		let content = m.content.clone();
		let sender = m.author_nick(&c.http).await.unwrap_or_else(|| m.author.name.clone());
		let existing_embeds = m.embeds.clone();
		let existing_attachments = m.attachments.clone();
		if existing_embeds.is_empty() && existing_attachments.is_empty() {
			if let Err(e) = m.delete(&c.http).await {
				eprintln!("Failed to delete message because {}", e);
			};
		} else {
			return
		}
		let uwu_sender = uwuify_str_sse(&sender);
		let uwuized = uwuify_str_sse(&content);
		let author_pfp = m.author.static_avatar_url();
		if let Err(e) = m.channel_id.send_message(&c.http, |m| {
			m.embed(|embed| {
				embed.author(|author| {
					let author = if let Some(url) = author_pfp {
						author.icon_url(url)
					} else {author};
					author.name(uwu_sender)
				}).description(uwuized)
			})
		}).await {
			eprintln!("Failed to send message because {}", e);
		}
	}
}
#![warn(clippy::pedantic)]

use serenity::{
	model::prelude::{Message, Ready},
	prelude::{Context, EventHandler, GatewayIntents},
	Client,
};
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
	let mut client = Client::builder(&token, intents)
		.event_handler(Handler)
		.await
		.expect("create client");

	// Run client
	if let Err(why) = client.start().await {
		eprintln!("Bot crash: {why}");
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
			eprintln!("Failed to start typing because {e}");
		}
		// Stop if the author is another bot.
		if m.author.bot {
			return;
		}

		let Ok(channel) = m.channel(&c.http).await else {
			return;
		};
		let Some(channel) = channel.guild() else {
			return;
		};
		let Some(topic) = channel.topic else { return };

		let filter = if topic.contains(UWU_STRING.unwrap_or(FALLBACK_UWU)) {
			uwuify_str_sse
		} else if topic.contains("!AAA") {
			|text: &str| {
				text.chars()
					.filter(|c: &char| match c {
						_ if c.is_whitespace() => true,
						_ if c.is_ascii_punctuation() => true,
						_ if !c.is_ascii() => false,
						_ if "AEIOUYaeiouy".contains(*c) => true,
						_ => false,
					})
					.collect()
			}
		} else {
			return;
		};

		// // Stop if the channel is not uwu
		// if !m.channel(&c.http).await.is_ok_and(|channel| {
		// 	channel.guild().is_some_and(|channel| {
		// 		channel
		// 			.topic
		// 			.is_some_and(|topic| topic.contains(UWU_STRING.unwrap_or(FALLBACK_UWU)))
		// 	})
		// }) {
		// 	return;
		// }

		let content = m.content.clone();
		let content: String = content
			.chars()
			.flat_map(|c| (c as u32).to_le_bytes())
			.filter(|b| b > &0)
			.map(|c| c as char)
			.collect();
		let sender = m
			.author_nick(&c.http)
			.await
			.unwrap_or_else(|| m.author.name.clone());
		let existing_embeds = m.embeds.clone();
		let existing_attachments = m.attachments.clone();
		let attachment_sizes: u64 = existing_attachments.iter().map(|a| a.size).sum();
		if existing_embeds.is_empty() && attachment_sizes < 0x1000 {
			if let Err(e) = m.delete(&c.http).await {
				eprintln!("Failed to delete message because {e}");
			};
		} else {
			return;
		}
		let uwu_sender = filter(&sender);
		let uwuized = filter(&content);
		let author_pfp = m.author.static_avatar_url();
		if let Err(e) = m
			.channel_id
			.send_message(&c.http, |m| {
				m.embed(|embed| {
					embed
						.author(|author| {
							let author = if let Some(url) = author_pfp {
								author.icon_url(url)
							} else {
								author
							};
							author.name(uwu_sender)
						})
						.description(uwuized)
				})
			})
			.await
		{
			eprintln!("Failed to send message because {e}");
		}
	}
}

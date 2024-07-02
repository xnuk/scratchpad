#![deny(clippy::all, clippy::pedantic)]

use std::env;
use std::io::{self, BufRead, Write};

use serde_json::value::Value;

const ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";

struct Config {
	max_token: u16,
	model: String,
	system_message: String,
	stream: bool,
	api_key: String,
}

fn get_nonstreaming(value: &Value) -> Option<&str> {
	value
		.as_object()?
		.get("choices")?
		.as_array()?
		.first()?
		.as_object()?
		.get("message")?
		.as_object()?
		.get("content")?
		.as_str()
}

fn get_streaming(value: &Value) -> Option<&str> {
	value
		.as_object()?
		.get("choices")?
		.as_array()?
		.first()?
		.as_object()?
		.get("delta")?
		.as_object()?
		.get("content")?
		.as_str()
}

fn main() -> anyhow::Result<()> {
	let query = env::args().skip(1).collect::<Box<[String]>>().join(" ");
	if query.is_empty() {
		anyhow::bail!("Usage: chatgpt [query ...]");
	}

	let config = Config {
		max_token: 300,
		model: "gpt-4o".into(),
		system_message: "Answer short as possible, but helpful.".into(),
		stream: true,
		api_key: env::var("OPENAI_API_KEY")
			.map_err(|_| anyhow::anyhow!("env var OPENAI_API_KEY not found"))?,
	};

	let body = serde_json::to_string(&serde_json::json!({
		"model": config.model,
		"messages": [
			{ "role": "system", "content": config.system_message },
			{ "role": "user", "content": query }
		],
		"max_tokens": config.max_token,
		"stream": config.stream,
	}))?;

	let response = ureq::post(ENDPOINT)
		.set("content-type", "application/json")
		.set("authorization", &format!("Bearer {}", config.api_key))
		.send_string(&body)?
		.into_reader();

	if config.stream {
		let mut stdout = io::stdout();

		let mut buf = io::BufReader::new(response);
		let mut data = String::new();
		let mut line = String::new();

		while buf.read_line(&mut line).is_ok() {
			if let Some(chunk) = line.strip_prefix("data: ") {
				data.push_str(chunk);
				data.push('\n');
			} else {
				if data.trim() == "[DONE]" {
					break;
				}

				let value = serde_json::from_str(&data)?;
				if let Some(chunk) = get_streaming(&value) {
					stdout.write_all(chunk.as_bytes())?;
					if buf.buffer().len() < 5 {
						stdout.flush()?;
					}
				}

				data.clear();
			}
			line.clear();
		}

		stdout.write_all(b"\n")?;
		stdout.flush()?;
		Ok(())
	} else {
		let body = serde_json::from_reader(response)?;
		if let Some(response) = get_nonstreaming(&body) {
			println!("{response}");
			Ok(())
		} else {
			Err(anyhow::anyhow!("no response"))
		}
	}
}

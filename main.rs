#![deny(clippy::all, clippy::pedantic)]

use std::env;
use std::io::{self, BufRead, Write};

use serde_json::value::Value;

trait Keyable {
	fn get(value: &Value, key: Self) -> Option<&Value>;
}

impl Keyable for usize {
	fn get(value: &Value, key: Self) -> Option<&Value> {
		value.as_array()?.get(key)
	}
}

impl Keyable for &str {
	fn get(value: &Value, key: Self) -> Option<&Value> {
		value.as_object()?.get(key)
	}
}

macro_rules! query {
	($value:expr, [$($key:expr),+ $(,)?]) => {
		Some($value).and_then(|v| {
			$( let v = Keyable::get(v, $key)?; )+
			Some(v)
		})
	};
}

const ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";

struct Config {
	max_token: u16,
	model: String,
	system_message: String,
	stream: bool,
	api_key: String,
}

fn get_nonstreaming(value: &Value) -> Option<&str> {
	query!(value, ["choices", 0, "message", "content"])?.as_str()
}

fn get_streaming(value: &Value) -> Option<&str> {
	query!(value, ["choices", 0, "delta", "content"])?.as_str()
}

fn main() -> anyhow::Result<()> {
	let query = env::args().skip(1).collect::<Box<[String]>>().join(" ");
	if query.is_empty() {
		anyhow::bail!("Usage: chatgpt [query ...]");
	}

	let config = Config {
		max_token: 300,
		model: "gpt-5-nano".into(),
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

	let agent: ureq::Agent = ureq::Agent::config_builder()
		.https_only(true)
		.http_status_as_error(false)
		.build()
		.into();

	let mut response = agent
		.post(ENDPOINT)
		.header("content-type", "application/json")
		.header("authorization", &format!("Bearer {}", config.api_key))
		.send(&body)?;
	let status_code = response.status();
	if !status_code.is_success() {
		let body = response.body_mut().read_to_string().unwrap_or_default();
		let message = serde_json::from_str(&body)
			.ok()
			.and_then(|v: Value| {
				let z = query!(&v, ["error", "message"])?.as_str()?;
				Some(z.to_string())
			})
			.unwrap_or(body);

		eprintln!("{message}");
		return Err(ureq::Error::StatusCode(status_code.as_u16()).into());
	}
	let body = response.body_mut().as_reader();

	if config.stream {
		let mut stdout = io::stdout();

		let mut buf = io::BufReader::new(body);
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
		let body = serde_json::from_reader(body)?;
		if let Some(response) = get_nonstreaming(&body) {
			println!("{response}");
			Ok(())
		} else {
			Err(anyhow::anyhow!("no response"))
		}
	}
}

use base64::{encode_config, URL_SAFE_NO_PAD};
use std::collections::HashMap;
use std::io::{self, Write};

struct UrlShortener {
    storage: HashMap<String, String>,
}

impl UrlShortener {
    fn new() -> Self {
        UrlShortener {
            storage: HashMap::new(),
        }
    }

    fn shorten_url(&mut self, original_url: &str) -> String {
        let short_alias = encode_config(original_url, URL_SAFE_NO_PAD);
        self.storage.insert(short_alias.clone(), original_url.to_string());
        short_alias
    }

    fn get_original_url(&self, short_alias: &str) -> Option<&String> {
        self.storage.get(short_alias)
    }

    fn list_urls(&self) {
        for (short_alias, original_url) in &self.storage {
            println!("{} -> {}", short_alias, original_url);
        }
    }
}

fn main() {
    let mut url_shortener = UrlShortener::new();

    loop {
        print!("Enter command (shorten/get/list/exit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input_parts: Vec<&str> = input.trim().split_whitespace().collect();

        if input_parts.is_empty() {
            continue;
        }

        match input_parts[0] {
            "shorten" => {
                if input_parts.len() < 2 {
                    println!("Usage: shorten <url>");
                } else {
                    let short_alias = url_shortener.shorten_url(input_parts[1]);
                    println!("Shortened URL: {}", short_alias);
                }
            }
            "get" => {
                if input_parts.len() < 2 {
                    println!("Usage: get <short_alias>");
                } else {
                    match url_shortener.get_original_url(input_parts[1]) {
                        Some(original_url) => println!("Original URL: {}", original_url),
                        None => println!("Short alias not found."),
                    }
                }
            }
            "list" => {
                url_shortener.list_urls();
            }
            "exit" => break,
            _ => println!("Unknown command. Use 'shorten', 'get', 'list', or 'exit'."),
        }
    }
}

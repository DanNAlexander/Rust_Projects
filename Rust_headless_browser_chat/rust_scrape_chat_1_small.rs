use scraper::{Html, Selector};
use reqwest::blocking::Client;

// Function to fetch chat from a given URL
fn fetch_chat(url: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let response = client.get(url).send()?;
    let body = response.text()?;
    Ok(body)
}

// Function to parse chat from the HTML content
fn parse_chat(html: &str, chat_selector: &str) -> Vec<String> {
    let fragment = Html::parse_fragment(html);
    let selector = Selector::parse(chat_selector).unwrap();

    let mut chat_messages = Vec::new();
    for element in fragment.select(&selector) {
        chat_messages.push(element.inner_html());
    }

    chat_messages
}

fn main() {
    let youtube_url = "https://www.youtube.com/watch?v=YOUR_VIDEO_ID";
    let odysee_url = "https://odysee.com/@CHANNEL_NAME/VIDEO_ID";

    let youtube_chat = fetch_chat(youtube_url).unwrap();
    let odysee_chat = fetch_chat(odysee_url).unwrap();

    let youtube_chat_selector = "your-youtube-chat-selector";
    let odysee_chat_selector = "your-odysee-chat-selector";

    let youtube_messages = parse_chat(&youtube_chat, youtube_chat_selector);
    let odysee_messages = parse_chat(&odysee_chat, odysee_chat_selector);

    // Now you can work with the chat messages from both platforms
    for message in youtube_messages {
        println!("YouTube Chat: {}", message);
    }

    for message in odysee_messages {
        println!("Odysee Chat: {}", message);
    }
}

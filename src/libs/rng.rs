use rand::Rng;
use rand::rngs::ThreadRng;
use uuid::Uuid;
use rand::rng;


/// Generates a random UUID as a `String`.
pub fn uuid() -> String {
    Uuid::new_v4().to_string()
}

/// Generates a scan ID with a prefix and UUID-based identifier.
pub fn scan_id() -> String {
    format!("v_{}", uuid().replace('-', "_"))
}

/// Returns a random User-Agent string from a predefined list.
pub fn user_agent() -> String {
    const USER_AGENTS: &[&str] = &[
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36",
        "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/537.36 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/537.36",
        "Mozilla/5.0 (iPad; CPU OS 16_0 like Mac OS X) AppleWebKit/537.36 (KHTML, like Gecko) Version/16.0 Mobile/15E148 Safari/537.36",
        "Mozilla/5.0 (Android 13; Mobile; rv:109.0) Gecko/109.0 Firefox/109.0",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:108.0) Gecko/20100101 Firefox/108.0",
        "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:107.0) Gecko/20100101 Firefox/107.0",
    ];

    let mut rng: ThreadRng = rng();
    let index = rng.random_range(0..USER_AGENTS.len());
    USER_AGENTS[index].to_string()
}

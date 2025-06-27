use clap::Parser;
use reqwest::blocking::Client;
use reqwest::Proxy;
use std::time::Duration;
use anyhow::{Result, Context};

/// Simple, private curl-like HTTP client
#[derive(Parser, Debug)]
#[command(name = "purl", about = "Private URL fetcher", version)]
struct Args {
    /// The URL to fetch (e.g., wttr.in or https://example.com)
    url: String,

    /// Allow insecure HTTP URLs
    #[arg(short = 'u', long)]
    unsecured: bool,

    /// Optional SOCKS5 proxy (e.g., socks5h://127.0.0.1:9050)
    #[arg(short, long)]
    proxy: Option<String>,

    /// Show response headers
    #[arg(short = 'H', long)]
    show_headers: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Sanitize and normalize the URL
    let clean_url = normalize_and_sanitize_url(&args.url)?;

    // Block HTTP unless explicitly allowed
    if clean_url.starts_with("http://") && !args.unsecured {
        eprintln!("❌ HTTP is insecure. Use --unsecured (-u) to allow it.");
        std::process::exit(1);
    }

    if clean_url.starts_with("http://") {
        eprintln!("⚠️ Warning: Insecure HTTP request. Your traffic may be visible to attackers.");
    }

    // Build client
    let mut builder = Client::builder()
    .timeout(Duration::from_secs(10))
    .user_agent("curl/8.0") // Pretend to be curl for plain text
    .redirect(reqwest::redirect::Policy::limited(5))
    .danger_accept_invalid_certs(false); // Enforce TLS verification

    if let Some(proxy_url) = args.proxy {
        builder = builder.proxy(Proxy::all(&proxy_url)?);
    }

    let client = builder.build().context("Failed to build HTTP client")?;

    // Send GET request with proper headers
    let response = client
    .get(&clean_url)
    .header("Accept", "text/plain")
    .header("Connection", "close")
    .send()
    .with_context(|| format!("Failed to fetch URL: {}", clean_url))?;

    // Print response headers (optional)
    if args.show_headers {
        for (key, value) in response.headers() {
            println!("{}: {}", key, value.to_str().unwrap_or("<binary>"));
        }
        println!();
    }

    // Output the body
    let body = response.text().context("Failed to read response body")?;
    println!("{}", body);

    Ok(())
}

/// Normalize and sanitize input URL:
/// - Add https:// if missing
/// - Strip known tracking parameters
fn normalize_and_sanitize_url(raw_url: &str) -> Result<String> {
    let mut url_str = raw_url.to_string();

    // Add scheme if missing
    if !url_str.starts_with("http://") && !url_str.starts_with("https://") {
        url_str = format!("https://{}", url_str);
    }

    let parsed = url::Url::parse(&url_str)
    .with_context(|| format!("Invalid URL: {}", url_str))?;

    // Remove tracking query parameters
    let keep_params = parsed.query_pairs()
    .filter(|(k, _)| {
        let k = k.to_ascii_lowercase();
        !k.starts_with("utm_")
        && k != "fbclid"
        && k != "gclid"
        && k != "mc_cid"
        && k != "mc_eid"
        && k != "ref"
        && k != "trk"
        && k != "aff"
        && k != "igshid"
        && k != "scid"
    })
    .collect::<Vec<_>>();

    let mut clean_url = parsed.clone();
    clean_url.set_query(Some(
        &url::form_urlencoded::Serializer::new(String::new())
        .extend_pairs(keep_params)
        .finish(),
    ));

    Ok(clean_url.to_string())
}

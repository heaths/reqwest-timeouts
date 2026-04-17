use clap::{Parser, Subcommand};
use std::time::{Duration, Instant};

#[derive(Parser)]
#[command(about = "Test reqwest ClientBuilder timeout settings")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Test ClientBuilder::timeout() — total request deadline.
    Timeout {
        /// Total timeout in seconds for the request.
        #[arg(long)]
        timeout: f64,

        /// How many seconds the server should delay before responding.
        #[arg(long)]
        server_delay: f64,

        /// Base URL of the test server.
        #[arg(long, default_value = "http://localhost:3000")]
        url: String,
    },

    /// Test ClientBuilder::read_timeout() — per-read idle timeout.
    ReadTimeout {
        /// Per-read timeout in seconds.
        #[arg(long)]
        read_timeout: f64,

        /// Seconds the server pauses between each chunk.
        #[arg(long)]
        chunk_delay: f64,

        /// Number of chunks the server sends.
        #[arg(long, default_value_t = 5)]
        chunks: u32,

        /// Base URL of the test server.
        #[arg(long, default_value = "http://localhost:3000")]
        url: String,
    },

    /// Test ClientBuilder::connect_timeout() — TCP connect phase timeout.
    ConnectTimeout {
        /// Connect timeout in seconds.
        #[arg(long)]
        connect_timeout: f64,

        /// Host to connect to. Use a non-routable address (e.g. 192.0.2.1:1234) to
        /// trigger a failure, or the test server address for success.
        #[arg(long, default_value = "http://localhost:3000")]
        host: String,
    },
}

fn secs(s: f64) -> Duration {
    Duration::from_secs_f64(s)
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Timeout {
            timeout,
            server_delay,
            url,
        } => run_timeout(timeout, server_delay, &url).await,

        Command::ReadTimeout {
            read_timeout,
            chunk_delay,
            chunks,
            url,
        } => run_read_timeout(read_timeout, chunk_delay, chunks, &url).await,

        Command::ConnectTimeout {
            connect_timeout,
            host,
        } => run_connect_timeout(connect_timeout, &host).await,
    }
}

async fn run_timeout(timeout: f64, server_delay: f64, base_url: &str) {
    println!("=== timeout test ===\n  timeout:      {timeout}s\n  server_delay: {server_delay}s");

    let client = reqwest::Client::builder()
        .timeout(secs(timeout))
        .build()
        .expect("failed to build client");

    let url = format!("{base_url}/delay?seconds={server_delay}");
    let start = Instant::now();

    match client.get(&url).send().await {
        Ok(resp) => match resp.text().await {
            Ok(body) => println!(
                "SUCCESS ({:.2}s): {} bytes\n  body: {}",
                start.elapsed().as_secs_f64(),
                body.len(),
                body.trim(),
            ),
            Err(e) => println!("FAILED ({:.2}s): {e}", start.elapsed().as_secs_f64()),
        },
        Err(e) => println!("FAILED ({:.2}s): {e}", start.elapsed().as_secs_f64()),
    }
}

async fn run_read_timeout(read_timeout: f64, chunk_delay: f64, chunks: u32, base_url: &str) {
    println!(
        "=== read-timeout test ===\n  read_timeout: {read_timeout}s\n  chunk_delay:  {chunk_delay}s\n  chunks:       {chunks}"
    );

    let client = reqwest::Client::builder()
        .read_timeout(secs(read_timeout))
        .build()
        .expect("failed to build client");

    let url = format!("{base_url}/slow-body?chunk_delay={chunk_delay}&chunks={chunks}");
    let start = Instant::now();

    match client.get(&url).send().await {
        Ok(resp) => match resp.bytes().await {
            Ok(body) => println!(
                "SUCCESS ({:.2}s): {} bytes",
                start.elapsed().as_secs_f64(),
                body.len(),
            ),
            Err(e) => println!("FAILED ({:.2}s): {e}", start.elapsed().as_secs_f64()),
        },
        Err(e) => println!("FAILED ({:.2}s): {e}", start.elapsed().as_secs_f64()),
    }
}

async fn run_connect_timeout(connect_timeout: f64, host: &str) {
    println!(
        "=== connect-timeout test ===\n  connect_timeout: {connect_timeout}s\n  host:            {host}"
    );

    let client = reqwest::Client::builder()
        .connect_timeout(secs(connect_timeout))
        .build()
        .expect("failed to build client");

    let url = if host.starts_with("http") {
        format!("{host}/health")
    } else {
        format!("http://{host}/health")
    };
    let start = Instant::now();

    match client.get(&url).send().await {
        Ok(resp) => match resp.text().await {
            Ok(body) => println!(
                "SUCCESS ({:.2}s): connected — {body}",
                start.elapsed().as_secs_f64(),
            ),
            Err(e) => println!("FAILED ({:.2}s): {e}", start.elapsed().as_secs_f64()),
        },
        Err(e) => println!("FAILED ({:.2}s): {e}", start.elapsed().as_secs_f64()),
    }
}

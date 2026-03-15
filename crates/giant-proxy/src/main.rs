mod client;

use clap::{Parser, Subcommand};
use client::DaemonClient;

#[derive(Parser)]
#[command(name = "giant-proxy", about = "HTTPS proxy with Map Remote rules")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    On {
        #[arg(long)]
        profile: Option<String>,
        #[arg(long)]
        system_proxy: bool,
        #[arg(long)]
        pac: bool,
    },
    Off,
    Status,
    Use {
        profile: String,
        #[arg(long)]
        also: Option<Vec<String>>,
        #[arg(long)]
        rule: Option<String>,
    },
    Toggle {
        rule_id: String,
    },
    Profiles,
    Doctor {
        #[arg(long)]
        fix: bool,
    },
    Env,
    Version,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = DaemonClient::new();

    match cli.command {
        Commands::Init => {
            println!("initializing giant-proxy...");
            let config_dir = dirs::home_dir()
                .expect("home directory must exist")
                .join(".giant-proxy");
            std::fs::create_dir_all(&config_dir).expect("failed to create config dir");
            std::fs::create_dir_all(config_dir.join("profiles"))
                .expect("failed to create profiles dir");
            std::fs::create_dir_all(config_dir.join("logs")).expect("failed to create logs dir");
            println!("config directory: {}", config_dir.display());
            println!("run `giant-proxy on` to start the proxy");
        }
        Commands::On { profile, .. } => {
            ensure_daemon(&client).await;
            let profile_name = profile.unwrap_or_else(|| "preprod".to_string());
            match client.post(&format!("/use/{}", profile_name), None).await {
                Ok(resp) => println!("proxy on: {}", resp),
                Err(e) => eprintln!("error: {}", e),
            }
        }
        Commands::Off => {
            if client.is_daemon_running() {
                match client.post("/stop", None).await {
                    Ok(resp) => println!("proxy off: {}", resp),
                    Err(e) => eprintln!("error: {}", e),
                }
            } else {
                println!("daemon not running");
            }
        }
        Commands::Status => {
            if !client.is_daemon_running() {
                println!("daemon not running");
                return;
            }
            match client.get("/status").await {
                Ok(resp) => println!("{}", serde_json::to_string_pretty(&resp).unwrap()),
                Err(e) => eprintln!("error: {}", e),
            }
        }
        Commands::Use { profile, .. } => {
            ensure_daemon(&client).await;
            match client.post(&format!("/use/{}", profile), None).await {
                Ok(resp) => println!("switched to {}: {}", profile, resp),
                Err(e) => eprintln!("error: {}", e),
            }
        }
        Commands::Toggle { rule_id } => {
            ensure_daemon(&client).await;
            match client
                .post(&format!("/rules/{}/toggle", rule_id), None)
                .await
            {
                Ok(resp) => println!("toggled: {}", resp),
                Err(e) => eprintln!("error: {}", e),
            }
        }
        Commands::Profiles => {
            ensure_daemon(&client).await;
            match client.get("/profiles").await {
                Ok(resp) => println!("{}", serde_json::to_string_pretty(&resp).unwrap()),
                Err(e) => eprintln!("error: {}", e),
            }
        }
        Commands::Doctor { .. } => {
            println!("running diagnostics...");
            let config_dir = dirs::home_dir()
                .expect("home directory must exist")
                .join(".giant-proxy");

            let ca_cert = config_dir.join("ca").join("giant-proxy-ca.pem");
            println!(
                "  CA cert: {}",
                if ca_cert.exists() { "found" } else { "MISSING" }
            );

            let ca_key = config_dir.join("ca").join("giant-proxy-ca-key.pem");
            println!(
                "  CA key:  {}",
                if ca_key.exists() { "found" } else { "MISSING" }
            );

            println!(
                "  daemon:  {}",
                if client.is_daemon_running() {
                    "running"
                } else {
                    "stopped"
                }
            );
        }
        Commands::Env => {
            if !client.is_daemon_running() {
                let config_dir = dirs::home_dir()
                    .expect("home directory must exist")
                    .join(".giant-proxy");
                let ca_path = config_dir.join("ca").join("giant-proxy-ca.pem");
                println!("export HTTP_PROXY=http://127.0.0.1:8080");
                println!("export HTTPS_PROXY=http://127.0.0.1:8080");
                println!("export NODE_EXTRA_CA_CERTS={}", ca_path.display());
                println!("export NO_PROXY=localhost,127.0.0.1");
            } else {
                match client.get("/env").await {
                    Ok(resp) => {
                        if let Some(snippet) = resp.get("shell_snippet").and_then(|s| s.as_str()) {
                            println!("{}", snippet);
                        }
                    }
                    Err(e) => eprintln!("error: {}", e),
                }
            }
        }
        Commands::Version => {
            println!("giant-proxy {}", env!("CARGO_PKG_VERSION"));
        }
    }
}

async fn ensure_daemon(client: &DaemonClient) {
    if client.is_daemon_running() && client.get("/health").await.is_ok() {
        return;
    }

    eprintln!("daemon not running. start it with: giantd --foreground");
    std::process::exit(1);
}

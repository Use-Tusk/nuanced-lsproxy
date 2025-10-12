use clap::Parser;

use log::{error, info};
use lsproxy::{
    api_types::SupportedLanguages, initialize_app_state_with_mount_dir,
    run_server_with_port_and_host, write_openapi_to_file,
};
use std::path::PathBuf;
use std::str::FromStr;

/// Command line interface for LSProxy server
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Write OpenAPI specification to openapi.json file
    #[arg(short, long)]
    write_openapi: bool,

    /// Host address to bind the server to
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    /// Override the default mount directory path where your workspace files are located
    #[arg(long)]
    mount_dir: Option<String>,

    /// Port number to bind the server to
    #[arg(long, default_value_t = 4444)]
    port: u16,

    /// Comma-separated list of languages to start (e.g., "python,golang").
    /// If not provided, will auto-detect languages from workspace files.
    /// Supported: python, typescript_javascript, rust, cpp, csharp, java, golang, php, ruby, ruby_sorbet
    #[arg(long)]
    languages: Option<String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up panic handler for better error reporting
    std::panic::set_hook(Box::new(|panic_info| {
        error!("Server panicked: {:?}", panic_info);
    }));

    // Initialize tracing subscriber for better logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    // Parse command line arguments
    let cli = Cli::parse();

    // Handle OpenAPI spec generation if requested
    if cli.write_openapi {
        if let Err(e) = write_openapi_to_file(&PathBuf::from("openapi.json")) {
            error!("Error: Failed to write the openapi.json to a file. Please see error for more details.");
            return Err(e);
        }
        return Ok(());
    }

    // Parse languages from CLI flag or environment variable
    let languages = parse_languages(cli.languages.or_else(|| std::env::var("LANGUAGES").ok()))?;

    // Initialize application state with optional mount directory override
    let app_state = initialize_app_state_with_mount_dir(cli.mount_dir.as_deref(), languages)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    // Run the server with specified host
    info!("Starting on port {}", cli.port);

    run_server_with_port_and_host(app_state, cli.port, &cli.host).await
}

/// Parse comma-separated language names into Vec<SupportedLanguages>
fn parse_languages(
    languages_str: Option<String>,
) -> std::io::Result<Option<Vec<SupportedLanguages>>> {
    let Some(languages_str) = languages_str else {
        return Ok(None);
    };

    let languages_str = languages_str.trim();
    if languages_str.is_empty() {
        return Ok(None);
    }

    let mut languages = Vec::new();
    let mut invalid_languages = Vec::new();

    for lang_str in languages_str.split(',') {
        let lang_str = lang_str.trim();
        match SupportedLanguages::from_str(lang_str) {
            Ok(lang) => languages.push(lang),
            Err(_) => invalid_languages.push(lang_str.to_string()),
        }
    }

    if !invalid_languages.is_empty() {
        let valid_languages = [
            "python",
            "typescript_javascript",
            "rust",
            "cpp",
            "csharp",
            "java",
            "golang",
            "php",
            "ruby",
            "ruby_sorbet",
        ];

        error!("Invalid language(s): {}", invalid_languages.join(", "));
        error!("\nSupported languages:");
        for lang in valid_languages {
            error!("  - {}", lang);
        }
        error!(
            "\nExample: --languages python,golang or LANGUAGES=python,golang"
        );

        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Invalid language(s): {}", invalid_languages.join(", ")),
        ));
    }

    Ok(Some(languages))
}

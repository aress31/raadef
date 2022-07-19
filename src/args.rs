use crate::constants::ENDPOINTS;

use chrono::Utc;
use clap::{Parser, __macro_refs::once_cell};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use std::path::PathBuf;

fn default_result_path() -> &'static str {
    static DEFAULT: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {
        let timestamp = Utc::now().format("%Y-%m-%dT%H.%M.%S");
        format!("./results/results-{}.txt", timestamp)
    });

    return &DEFAULT;
}

#[derive(Debug, Parser)]
#[clap(
    author = "Alexandre Teyar - @aress31",
    version,
    about = "This program is a exploitation framework designed to assist red teamers in their attempts to obtain a foothold into \
            targets' AzureAD.",
    long_about = None
)]
pub struct Args {
    #[clap(
        action,
        help = "continues authentication attempts even after successes",
        default_value_t = false,
        long,
        required = false,
        short
    )]
    pub continue_on_success: bool,
    #[clap(
        default_value_t = 0,
        help = "sets a delay (in seconds) between each authentication attempt",
        long,
        required = false,
        short
    )]
    pub delay: u64,
    #[clap(
        default_value = "https://login.microsoft.com/common/oauth2/token",
        help = "authentication endpoint",
        long,
        required = false,
        short
    )]
    pub endpoint: String,
    #[clap(
        action,
        help = "uses HTTP/2.0 for better performance",
        default_value_t = false,
        long,
        required = false
    )]
    pub http_2: bool,
    #[clap(
        default_value_t = 2000,
        help = "sets a random delay (in seconds) between 0 and <JITTER> between each authentication attempt",
        long,
        required = false,
        short
    )]
    pub jitter: u64,
    #[clap(
        default_value_t = 1800,
        help = "sets a delay (in seconds) between each <LOOP_NUMBER>",
        long,
        required = false
    )]
    pub loop_delay: u64,
    #[clap(
        default_value_t = 0,
        help = "sets a random delay (in seconds) between 0 and <LOOP_JITTER> between each <LOOP_NUMBER>",
        long,
        required = false
    )]
    pub loop_jitter: u64,
    #[clap(
        default_value_t = 5,
        help = "sets the number of outer loops to iterate before waiting for <LOOP_DELAY>",
        long,
        required = false
    )]
    pub loop_number: u64,
    #[clap(
        default_value = default_result_path(),
        help = "defines outfile for program output",
        long,
        required = false,
        short
    )]
    pub outfile: PathBuf,
    #[clap(help = "file containing passwords", long, required = true, short)]
    pub password: PathBuf,
    #[clap(
        help = "sets a proxy, e.g., http://localhost:8080",
        long,
        required = false,
        short = 'x'
    )]
    pub proxy: Option<String>,
    #[clap(arg_enum, default_value = "MSGraphApi", help = "ressource principal to authenticate to", long, required = false, short, value_parser = clap::builder::PossibleValuesParser::new(ENDPOINTS.keys()))]
    pub resource_principal: String,
    #[clap(help = "tenant to authenticate to", long, required = false, short)]
    pub tenant: Option<String>,
    #[clap(help = "file containing usernames", long, required = true, short)]
    pub username: String,
    #[clap(flatten)]
    pub verbose: Verbosity<InfoLevel>,
}

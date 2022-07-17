use clap::{ArgEnum, Parser};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use std::path::PathBuf;

#[derive(ArgEnum, Clone, Debug, strum_macros::Display)]
pub enum Resource {
    AadGraphApi,
    AzureKeyVault,
    AzureMgmtApi,
    CloudWebAppProxy,
    Database,
    DataCatalog,
    MSGraphApi,
    MSMAMService,
    O365Exchange,
    O365Yammer,
    OfficeApps,
    OfficeMgmt,
    OneNote,
    Outlook,
    Sara,
    Skype4Business,
    SpacesApi,
    WebShellSuite,
    WindowsNetMgmtApi,
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
        help = "sets a delay in seconds between each connection",
        long,
        required = false,
    )]
    pub delay: u64,
    #[clap(
        default_value_t = 0,
        help = "sets a random delay between each connection",
        long,
        required = false,
        short
    )]
    pub jitter: u64,
    #[clap(help = "file containing passwords", long, required = true, short)]
    pub password: PathBuf,
    #[clap(
        default_value = "results.log",
        help = "defines outfile for program output",
        long,
        required = false,
        short
    )]
    pub outfile: PathBuf,
    #[clap(
        default_value = "",
        help = "sets a proxy, e.g., http://localhost:8080",
        long,
        required = false,
        short = 'x'
    )]
    pub proxy: String,
    #[clap(arg_enum, default_value_t = Resource::MSGraphApi, help = "ressource principal to authenticate to", long, required = false, short)]
    pub resource_principal: Resource,
    #[clap(help = "tenant to authenticate to", long, required = true, short)]
    pub tenant: String,
    #[clap(help = "file containing usernames", long, required = true, short)]
    pub username: String,
    #[clap(flatten)]
    pub verbose: Verbosity<InfoLevel>,
}

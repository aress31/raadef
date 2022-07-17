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
    about = "This program is designed to assist red teamers in their attempts to obtain a foothold into targets' AzureAD using \
            \"Password Attacks\". \
            The supported features include password spraying as well as brute forcing with fine-tuning of the attack settings \
            in order to help reducing the likeliness of locking out targetted accounts.",
    long_about = None
)]
pub struct Args {
    #[clap(action, default_value_t = false, long, required = false, short)]
    pub continue_on_success: bool,
    #[clap(default_value_t = 4, long, required = false, short)]
    pub delay: u64,
    // Forces the spray to continue and not stop when multiple account lockouts are detected.
    // #[clap(action, default_value_t = false, long, required = false, short)]
    // force: bool,
    #[clap(long, required = true, short)]
    pub password: PathBuf,
    // #[clap(default_value_t = 5, help ="test", long, required = false, short)]
    // threads: u8,
    #[clap(default_value = "results.log", long, required = false, short)]
    pub outfile: PathBuf,
    #[clap(default_value = "", long, required = false, short = 'x')]
    pub proxy: String,
    #[clap(arg_enum, default_value_t = Resource::MSGraphApi, long, required = false, short)]
    pub resource_principal: Resource,
    #[clap(long, required = true, short)]
    pub tenant: String,
    #[clap(long, required = true, short)]
    pub username: String,
    #[clap(flatten)]
    pub verbose: Verbosity<InfoLevel>,
}

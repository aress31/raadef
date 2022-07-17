use crate::structs::Foo;
use phf::{phf_map, Map};

pub static ENDPOINTS: Map<&'static str, &'static str> = phf_map! {
    "AadGraphApi" => "https://graph.windows.net",
    "AzureKeyVault" =>"https://vault.azure.net" ,
    "AzureMgmtApi" => "https://management.azure.com",
    "CloudWebAppProxy" => "https://proxy.cloudwebappproxy.net/registerapp",
    "Database" => "https://database.windows.net",
    "DataCatalog" => "https://datacatalog.azure.com",
    "MSGraphApi" => "https://graph.microsoft.com",
    "MSMAMService" => "https://msmamservice.api.application",
    "O365Exchange" => "https://outlook-sdf.office.com",
    "O365Yammer" => "https://api.yammer.com",
    "OfficeApps" => "https://officeapps.live.com",
    "OfficeMgmt" => "https://manage.office.com",
    "OneNote" => "https://onenote.com",
    "Outlook" => "https://outlook.office365.com",
    "Sara" => "https://api.diagnostics.office.com",
    "Skype4Business" => "https://api.skypeforbusiness.com",
    "SpacesApi" => "https://api.spaces.skype.com",
    "WebShellSuite" => "https://webshell.suite.office.com",
    "WindowsNetMgmtApi" => "https://management.core.windows.net",
};

pub const ERROR_CODES: [Foo; 12] = [
    Foo {
        code: Some("AADSTS50076"),
        message: "ENABLED_MFA (AADSTS50076)",
        r#type: "PARTIAL_SUCCESS",
    },
    Foo {
        code: Some("AADSTS500011"),
        message: "INVALID_RESOURCE_PRINCIPAL (AADSTS500011)",
        r#type: "PARTIAL_SUCCESS",
    },
    // ---
    Foo {
        code: Some("AADSTS50057"),
        message: "DISABLED_ACCOUNT (AADSTS50057)",
        r#type: "FAILURE",
    },
    Foo {
        code: Some("AADSTS50158"),
        message: "CONDITIONAL_ACCESS (AADSTS50158)",
        r#type: "FAILURE",
    },
    Foo {
        code: Some("AADSTS50079"),
        message: "ENABLED_MFA (AADSTS50079)",
        r#type: "FAILURE",
    },
    Foo {
        code: Some("AADSTS50055"),
        message: "EXPIRED_PASWORD (AADSTS50055)",
        r#type: "FAILURE",
    },
    Foo {
        code: Some("AADSTS50126"),
        message: "INVALID_PASSWORD (AADSTS50126)",
        r#type: "FAILURE",
    },
    Foo {
        code: Some("AADSTS50059"),
        message: "INVALID_TENANT (AADSTS50059)",
        r#type: "FAILURE",
    },
    Foo {
        code: Some("AADSTS50128"),
        message: "INVALID_TENANT (AADSTS50128)",
        r#type: "FAILURE",
    },
    Foo {
        code: Some("AADSTS900023"),
        message: "INVALID_TENANT (AADSTS900023)",
        r#type: "FAILURE",
    },
    Foo {
        code: Some("AADSTS50034"),
        message: "INVALID_USERNAME (AADSTS50034)",
        r#type: "FAILURE",
    },
    Foo {
        code: Some("AADSTS50053"),
        message: "LOCKED_ACCOUNT (AADSTS50053)",
        r#type: "FAILURE",
    },
];

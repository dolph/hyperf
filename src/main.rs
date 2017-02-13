#[macro_use] extern crate log;
extern crate env_logger;

extern crate argparse;
extern crate hyper;
extern crate time;

struct Options {
    verbose: bool,
    url: String,
    requests: u64,
}

fn time_request(url: &String) -> f64 {
    let client = hyper::Client::new();

    let request = client.get(url);

    let start = time::precise_time_s();
    let wrapped_response = request.send();
    let end = time::precise_time_s();

    let response = wrapped_response.unwrap();

    println!("HTTP {}", response.status);

    return end - start;
}

fn benchmark(url: String, requests: u64) {
    for x in 0..requests {
        let duration = time_request(&url);

        info!("Duration: {} seconds", duration);

        let formatted_duration = format!("{:.*}", 3, 1000.0 * duration);
        println!("Duration: {} milliseconds", formatted_duration);
    }
}

fn main() {
    // Initialize logging.
    env_logger::init().unwrap();

    // Initialize options.
    let mut options = Options {
        verbose: false,
        url: "".to_string(),
        requests: 1,
    };

    // Parse command line arguments.
    {
        let mut parser = argparse::ArgumentParser::new();
        parser.set_description(env!("CARGO_PKG_DESCRIPTION"));
        parser.add_option(&["--version"],
            argparse::Print(
                format!(
                    "{} {}",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION"))),
            "Show version information.");
        parser.refer(&mut options.verbose)
            .add_option(&["-v", "--verbose"], argparse::StoreTrue,
            "Enable verbose output.");
        parser.refer(&mut options.requests)
            .add_option(&["-n", "--requests"], argparse::Store,
            "Number of requests to perform.");
        parser.refer(&mut options.url)
            .add_argument("url", argparse::Store, "URL to request.");
        parser.parse_args_or_exit();
    }

    debug!("verbose={}", options.verbose);
    debug!("url={}", options.url);

    println!("GET {}", options.url);

    benchmark(options.url, options.requests);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

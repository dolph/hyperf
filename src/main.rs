#[macro_use] extern crate log;
extern crate env_logger;

extern crate argparse;
extern crate hyper;
extern crate time;

struct Options {
    verbose: bool,
    url: String,
    requests: usize,
}

fn time_request(url: &String) -> f64 {
    let client = hyper::Client::new();

    let request = client.get(url);

    let start = time::precise_time_s();
    let wrapped_response = request.send();
    let end = time::precise_time_s();

    let response = wrapped_response.unwrap();

    info!("HTTP {}", response.status);
    info!("Duration: {} seconds", end - start);

    return end - start;
}

fn benchmark(url: String, requests: usize) {
    let mut total_duration = 0.0;

    for x in 0..requests {
        total_duration = total_duration + time_request(&url);

        if (x + 1) % 100 == 0 {
            println!("Completed {} requests...", x + 1);
        }
    }

    println!("Completed requests: {}", requests);

    let requests_float = requests as f64;
    let mean = total_duration / requests_float;

    let formatted_mean = format!("{:.*}", 3, 1000.0 * mean);
    println!("Mean response time: {} milliseconds", formatted_mean);

    let formatted_rps = format!("{:.*}", 3, 1.0 / mean);
    println!("Requests per second: {}", formatted_rps);
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

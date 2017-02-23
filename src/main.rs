use std::thread;
use std::process;

#[macro_use] extern crate log;
extern crate env_logger;

extern crate argparse;
extern crate hyper;
extern crate time;

use std::str::FromStr;

#[derive(Clone)]
struct Options {
    verbose: bool,
    method: String,
    url: String,
    body: String,
    requests: usize,
    concurrency: usize,
}

struct Statistics {
    requests: usize,
    total_duration: f64,
    errors: usize,
}

fn spawn_benchmark_thread(options: &Options, requests: &usize) -> thread::JoinHandle<Statistics> {
    let options_clone = options.clone();
    let requests_clone = requests.clone();

    let child = thread::spawn(move || {
        let client = hyper::Client::new();
        // client.set_read_timeout();
        // client.set_write_timeout();

        let mut stats = Statistics {
            requests: 0,
            total_duration: 0.0,
            errors: 0,
        };

        for _ in 0..requests_clone {
            let method = match hyper::method::Method::from_str(&options_clone.method) {
                Ok(method) => method,
                Err(_) => {
                    println!("Unsupported HTTP method.");
                    process::exit(1);
                },
            };

            // https://hyper.rs/hyper/v0.10.0/hyper/client/struct.RequestBuilder.html
            let request = client.request(method, &options_clone.url).body(&options_clone.body);
            // request.header(header);

            let start_time = time::precise_time_s();
            match request.send() {
                Ok(response) => response,
                Err(_) => {
                    stats.errors += 1;
                    continue;
                }
            };
            let end_time = time::precise_time_s();

            stats.requests += 1;
            stats.total_duration += end_time - start_time;
        }

        return stats;
    });

    return child;
}

fn benchmark(options: Options) {
    // Spawn child threads.
    let mut children = Vec::new();
    let requests_per_child = options.requests / options.concurrency;
    for _ in 0..options.concurrency {
        children.push(spawn_benchmark_thread(&options, &requests_per_child));
    }

    // Aggregate results from child threads.
    let mut stats = Statistics {
        requests: 0,
        total_duration: 0.0,
        errors: 0,
    };
    while let Some(child) = children.pop() {
        let results = child.join().unwrap();
        stats.requests += results.requests;
        stats.total_duration += results.total_duration;
    }

    print_report(options, stats);
}

fn print_report(options: Options, stats: Statistics) {
    println!("Successful requests: {}", stats.requests);
    println!("Errored requests: {}", stats.errors);
    println!("Concurrency: {}", options.concurrency);

    let mean = stats.total_duration / (stats.requests as f64);

    let formatted_mean = format!("{:.*}", 3, 1000.0 * mean);
    println!("Mean response time: {} milliseconds", formatted_mean);

    let formatted_rps = format!("{:.*}", 3, (options.concurrency as f64) / mean);
    println!("Requests per second: {}", formatted_rps);
}

fn main() {
    // Initialize logging.
    env_logger::init().unwrap();

    // Initialize options.
    let mut options = Options {
        verbose: false,
        method: "GET".to_string(),
        url: "http://localhost/".to_string(),
        body: "".to_string(),
        requests: 1,
        concurrency: 1,
    };

    // Parse command line arguments.
    {
        let mut parser = argparse::ArgumentParser::new();
        parser.set_description(env!("CARGO_PKG_DESCRIPTION"));
        parser.add_option(&["--version"],
            argparse::Print(
                format!(
                    "{}",
                    env!("CARGO_PKG_VERSION"))),
            "Show version information.");
        parser.refer(&mut options.verbose)
            .add_option(
                &["-v", "--verbose"],
                argparse::StoreTrue,
                "Enable verbose output.");
        parser.refer(&mut options.concurrency)
            .add_option(
                &["-c", "--concurrency"],
                argparse::Store,
                "Number of requests to perform in parallel (concurrent users).");
        parser.refer(&mut options.requests)
            .add_option(
                &["-n", "--requests"],
                argparse::Store,
                "Number of requests to perform.");
        parser.refer(&mut options.method)
            .add_argument(
                "method",
                argparse::Store,
                "HTTP method to use when requesting URL.")
            .required();
        parser.refer(&mut options.url)
            .add_argument(
                "url",
                argparse::Store,
                "URL to request.")
            .required();
        parser.refer(&mut options.body)
            .add_argument(
                "body",
                argparse::Store,
                "Body of the HTTP request, if applicable.");
        parser.parse_args_or_exit();
    }

    // Normalize input.
    options.method = options.method.to_uppercase();

    debug!("verbose={}", options.verbose);
    debug!("concurrency={}", options.concurrency);
    debug!("requests={}", options.requests);
    debug!("method={}", options.method);
    debug!("url={}", options.url);

    println!("{} {}", options.method, options.url);

    if options.requests % options.concurrency != 0 {
        println!("The number of requests to perform must be evenly divisible by the concurrency.");
        process::exit(1);
    }

    benchmark(options);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

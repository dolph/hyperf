use std::thread;
use std::process;

#[macro_use] extern crate log;
extern crate env_logger;

extern crate argparse;
extern crate hyper;
extern crate time;

struct Options {
    verbose: bool,
    url: String,
    requests: usize,
    concurrency: usize,
}

struct Statistics {
    requests: usize,
    total_duration: f64,
    errors: usize,
}

fn spawn_benchmark_thread(url: &String, requests: &usize) -> thread::JoinHandle<Statistics> {
    let url_clone = url.clone();
    let requests_clone = requests.clone();
    let child = thread::spawn(move || {
        let client = hyper::Client::new();

        let mut stats = Statistics {
            requests: requests_clone,
            total_duration: 0.0,
            errors: 0,
        };

        for _ in 0..requests_clone {
            let request = client.get(&url_clone);

            let start_time = time::precise_time_s();
            let wrapped_response = match request.send() {
                Ok(response) => response,
                Err(e) => {
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
        children.push(spawn_benchmark_thread(&options.url, &requests_per_child));
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
    println!("Completed requests: {}", stats.requests);
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
        url: "".to_string(),
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
                    "{} {}",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION"))),
            "Show version information.");
        parser.refer(&mut options.verbose)
            .add_option(&["-v", "--verbose"], argparse::StoreTrue,
            "Enable verbose output.");
        parser.refer(&mut options.concurrency)
            .add_option(&["-c", "--concurrency"], argparse::Store,
            "Number of requests to perform in parallel (concurrent users).");
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

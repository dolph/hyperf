#[macro_use] extern crate log;
extern crate env_logger;

extern crate argparse;
extern crate hyper;

struct Options {
    verbose: bool,
    url: String,
}

fn benchmark(url: String) {
    let client = hyper::Client::new();

    let res = client.get(&url).send().unwrap();

    println!("HTTP {}", res.status);
}

fn main() {
    // Initialize logging.
    env_logger::init().unwrap();

    // Initialize options.
    let mut options = Options {
        verbose: false,
        url: "".to_string(),
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
        parser.refer(&mut options.url)
            .add_argument("url", argparse::Store, "URL to request.");
        parser.parse_args_or_exit();
    }

    debug!("verbose={}", options.verbose);
    debug!("url={}", options.url);

    println!("GET {}", options.url);

    benchmark(options.url);
}

#[cfg(test)]
mod tests {
    use super::main;

    #[test]
    fn it_works() {
        main();
    }
}

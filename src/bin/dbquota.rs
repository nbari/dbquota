use clap::{App, Arg};
use dbquota::queries;
use std::{
    process, thread,
    time::{Duration, Instant},
};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::with_name("DSN").env("DSN"))
        .get_matches();

    let dsn = match matches.value_of("DSN") {
        Some(s) => s,
        _ => {
            eprintln!("could not find DSN");
            process::exit(1);
        }
    };

    let opts = mysql::Opts::from_url(dsn).unwrap_or_else(|e| {
        eprintln!("verify the DSN: {}", e);
        process::exit(1);
    });
    let mut opts = mysql::OptsBuilder::from_opts(opts);
    opts.stmt_cache_size(0);
    let pool = mysql::Pool::new_manual(1, 2, opts).unwrap_or_else(|e| {
        eprintln!("Could not connect: {}", e);
        process::exit(1);
    });

    loop {
        let start = Instant::now();
        let wait_time = Duration::from_secs(10);
        let q = queries::new(&pool);

        match q.update_db_size() {
            Err(e) => {
                eprintln!("Error: {}", e);
            }
            Ok(_) => {}
        }

        let runtime = start.elapsed();
        if let Some(remaining) = wait_time.checked_sub(runtime) {
            println!("sleeping for: {:?}", remaining);
            thread::sleep(remaining);
        }
    }
}

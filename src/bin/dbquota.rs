use clap::{App, Arg};
use dbquota::queries;
use std::{
    process, thread,
    time::{Duration, Instant},
};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("DSN")
                .env("DSN")
                .help("mysql://<username>:<password>@<host>:<port>/<database>")
                .long("dsn")
                .required(true)
                .short("d"),
        )
        .arg(
            Arg::with_name("every")
                .default_value("180")
                .help("run every N seconds")
                .long("every")
                .short("e")
                .validator(is_num),
        )
        .get_matches();

    let every = matches.value_of("every").unwrap().parse::<u64>().unwrap();
    let opts = mysql::Opts::from_url(matches.value_of("DSN").unwrap()).unwrap_or_else(|e| {
        eprintln!("verify the DSN: {}", e);
        process::exit(1);
    });
    let mut opts = mysql::OptsBuilder::from_opts(opts);
    opts.stmt_cache_size(0);
    let pool = mysql::Pool::new_manual(1, 5, opts).unwrap_or_else(|e| {
        eprintln!("Could not connect: {}", e);
        process::exit(1);
    });

    loop {
        let start = Instant::now();
        let wait_time = Duration::from_secs(every);
        let q = queries::new(&pool);

        match q.update_db_size() {
            Err(e) => eprintln!("Error Updating db size: {}", e),
            Ok(_) => {}
        };

        match q.enforce_quota() {
            Err(e) => eprintln!("Error enforcing quota: {}", e),
            Ok(_) => {}
        };

        let runtime = start.elapsed();
        if let Some(remaining) = wait_time.checked_sub(runtime) {
            println!("sleeping for: {:?}", remaining);
            thread::sleep(remaining);
        }
    }
}

fn is_num(s: String) -> Result<(), String> {
    if let Err(..) = s.parse::<u64>() {
        return Err(String::from("Not a valid number!"));
    }
    Ok(())
}

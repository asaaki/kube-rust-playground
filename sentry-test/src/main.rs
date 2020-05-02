#![forbid(unsafe_code)]
//#![deny(warnings)]

use dipstick::*;
use lazy_static::lazy_static;
use sentry::capture_message;

// pulls in APP_{NAME,VERSION,RELEASE}; RELEASE: NAME@VERSION
include!(concat!(env!("OUT_DIR"), "/app_data.rs"));

metrics! { PROXY: Proxy = "metrics_proxy" => {
        pub COUNTER: Counter = "my-counter";
        pub MARKER: Marker = "my-marker"; // monotonic counter (inc by 1)
        pub GAUGE: Gauge = "my-gauge";
        pub TIMER: Timer = "my-timer";
    }
}

lazy_static! {
    static ref SAMPLE: &'static str = "sample static";
}

fn main() {
    let _dsn = sentry_dsn(); // hard test for the env var
    let _sentry = sentry::init(sentry_options());
    configure_sentry();
    configure_dipstick();
    run_dipstick_test();
    println!("basic output to stdout");

    capture_message("Low Leve Debug", sentry::Level::Debug);
    capture_message("Info Level Message", sentry::Level::Info);
    capture_message("Warn Level Issue", sentry::Level::Warning);
    capture_message("Error Message Style", sentry::Level::Error);
    panic!("Everything is on fire!"); // uses sentry::Level::Fatal
}

fn run_dipstick_test() {
    ThreadLabel::set("test-thread-label", "value-thread");
    TIMER.time(|| {
        println!("-- step 1");
        COUNTER.count(3);
        GAUGE.value(50.0);
        MARKER.mark();
        std::thread::sleep(std::time::Duration::from_millis(12));
    });
    std::thread::sleep(std::time::Duration::from_secs(2));
    TIMER.time(|| {
        println!("-- step 2");
        COUNTER.count(1);
        GAUGE.value(42.0);
        MARKER.mark();
        MARKER.mark();
        std::thread::sleep(std::time::Duration::from_millis(23));
    });
    std::thread::sleep(std::time::Duration::from_secs(2));
    TIMER.time(|| {
        println!("-- step 3");
        COUNTER.count(9);
        GAUGE.value(99.0);
        MARKER.mark();
        std::thread::sleep(std::time::Duration::from_millis(9));
    });
    std::thread::sleep(std::time::Duration::from_secs(1));
    TIMER.interval_us(1_234); // should be reported as 1 (ms)
}

fn sentry_options() -> sentry::ClientOptions {
    sentry::ClientOptions {
        release: Some(APP_RELEASE.into()),
        environment: Some(sentry_environment().into()),
        // debug: true,
        ..Default::default()
    }
}

fn configure_sentry() {
    sentry::configure_scope(|scope| {
        scope.set_tag("profile", APP_PROFILE); // since we override environment
        scope.set_tag("target", APP_TARGET);
        // scope.set_user(Some(sentry::User {
        //     id: Some("none".into()),
        //     ..Default::default()
        // }));
        scope.set_tag("custom-tag-1", "fresh value");
        let extra = serde_json::json!({ "some": { "extra": { "data": 42 } } });
        scope.set_extra("custom-extra", extra);
    });
    sentry::integrations::panic::register_panic_handler();
}

fn sentry_dsn() -> String {
    std::env::var("SENTRY_DSN").expect("env var SENTRY_DSN not found, but it is required")
}

fn sentry_environment() -> String {
    std::env::var("SENTRY_ENVIRONMENT").unwrap_or_else(|_| "development".into())
}

fn configure_dipstick() {
    let mut targets = MultiOutput::new().named(APP_NAME);

    targets = targets.add_target(Stream::to_stdout());

    let prometheus = Prometheus::push_to("http://127.0.0.1:9091/metrics/job/dipstick_example")
        .expect("Prometheus socket issue");
    targets = targets.add_target(prometheus);

    if let Some(statsd_uri) = statsd_uri() {
        let statsd = Statsd::send_to(statsd_uri)
            .expect("Statsd socket issue")
            .named("for-statsd");
        targets = targets.add_target(statsd);
    }

    // Create the bucket and drain targets
    let bucket = AtomicBucket::new();
    bucket.drain(targets);
    // Crucial, set the flush interval, otherwise risk hammering targets
    bucket.flush_every(std::time::Duration::from_secs(1));
    PROXY.target(bucket); // or: bucket.named("toplevel-name")

    AppLabel::set("test-app-label", "value-app");
}

/// Only if the STATSD_HOST is present we return a complete URI
fn statsd_uri() -> Option<String> {
    match (statsd_host(), statsd_port()) {
        (Some(host), port) => Some(format!("{}:{}", host, port)),
        _ => None,
    }
}

fn statsd_host() -> Option<String> {
    std::env::var("STATSD_HOST").ok()
}

/// Returns env var value or the default
fn statsd_port() -> String {
    std::env::var("STATSD_PORT").unwrap_or_else(|_| "8125".into())
}

#![warn(rust_2018_idioms)]

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use async_std::{
    io::Error,
    prelude::*,
    sync::{Arc, RwLock},
};
use lazy_static::lazy_static;
use prometheus::{
    self, register_histogram_vec, register_int_counter, register_int_counter_vec, Encoder,
    HistogramVec, IntCounter, IntCounterVec, TextEncoder,
};
use std::{future::Future, pin::Pin, time::Instant};
use tide::{Middleware, Next, Request, Result as TideResult};

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

lazy_static! {
    static ref REQUEST_COUNTER: IntCounterVec = register_int_counter_vec!(
        "request_count",
        "My awesome i64 counter",
        &["status"]
    ).expect("metric registration failed");

    static ref REQUEST_FAILURES: IntCounter = register_int_counter!(
        "request_failures_all",
        "Counts request with resulted in processing failures (all apps)"
    ).expect("metric registration failed");

    static ref REQUEST_HISTOGRAM: HistogramVec = {
        register_histogram_vec!(
        "request_histogram_usecs",
        "My awesome histogram",
        &["status"],
        vec![ // adjust bucket according to expected observation ranges
            1.0,
            2.5,
            5.0,
            10.0,
            25.0,
            50.0,
            100.0,
            250.0,
            500.0,
            1_000.0,
            5_000.0,
            10_000.0,
            50_000.0,
            100_000.0,
            500_000.0,
            1_000_000.0,
            ]
    ).expect("metric registration failed")};
}

struct State {
    pub text: String,
}

type GlobalState = Arc<RwLock<State>>;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    inner_main().await
}

#[inline]
async fn inner_main() -> std::io::Result<()> {
    femme::with_level(tide::log::Level::Info.to_level_filter());
    let global = Arc::new(RwLock::new(State {
        text: "no text".to_string(),
    }));

    let app = main_app(global.clone());
    let prom = prom_app();
    app.try_join(prom).await?;
    Ok(())
}

async fn main_app(state: GlobalState) -> Result<(), Error> {
    let mut app = tide::with_state(state);
    app.middleware(MetricsMiddleware::new());
    app.at("/").get(handler);
    app.at("*").all(handler);
    app.listen("127.0.0.1:8001").await?;
    Ok(())
}

async fn prom_app() -> Result<(), Error> {
    let mut app = tide::new();
    app.at("/metrics").get(prom_reporter);
    app.listen("127.0.0.1:9184").await?;
    Ok(())
}

async fn handler(_req: Request<GlobalState>) -> TideResult {
    // let state = &req.state();
    // {
    //     // inner block to unlock for the following write
    //     let s = state.read().await;
    //     s.text;
    // }
    // {
    //     let mut s = state.write().await;
    //     s.text = String::from("some text in global state");
    // }
    Ok("Ok\n".into())
}

async fn prom_reporter(_req: Request<()>) -> TideResult {
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    let output = String::from_utf8(buffer).unwrap();
    Ok(output.into())
}

// ===== custom middlewares =====

#[derive(Debug, Clone)]
struct MetricsMiddleware {
    _priv: (),
}

impl MetricsMiddleware {
    fn new() -> Self {
        Self { _priv: () }
    }

    async fn measure<'a, State: Send + Sync + 'static>(
        &'a self,
        ctx: Request<State>,
        next: Next<'a, State>,
    ) -> tide::Result {
        // let path = ctx.uri().path().to_owned();
        // let method = ctx.method().to_string();
        let start = Instant::now();
        match next.run(ctx).await {
            Ok(res) => {
                let status = res.status().to_string();
                let duration = start.elapsed().as_secs_f64() * 1_000_000.0;
                REQUEST_HISTOGRAM
                    .with_label_values(&[&status])
                    .observe(duration);
                REQUEST_COUNTER.with_label_values(&[&status]).inc();
                Ok(res)
            }
            Err(err) => {
                REQUEST_FAILURES.inc();
                Err(err)
            }
        }
    }
}

impl<State: Send + Sync + 'static> Middleware<State> for MetricsMiddleware {
    fn handle<'a>(
        &'a self,
        ctx: Request<State>,
        next: Next<'a, State>,
    ) -> BoxFuture<'a, tide::Result> {
        Box::pin(async move { self.measure(ctx, next).await })
    }
}

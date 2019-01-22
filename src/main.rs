extern crate actix_web;
use actix_web::{server, App, HttpRequest};
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNT: AtomicUsize = AtomicUsize::new(0);

fn index(_req: &HttpRequest) -> String {
    COUNT.fetch_add(1, Ordering::SeqCst);
    format!("{:?} ", COUNT)
}

fn main() {
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind("127.0.0.1:8000")
        .unwrap()
        .run();
}

#[macro_use] extern crate criterion;
use dsrv::server;
use criterion::Criterion;

pub fn init_server(c: &mut Criterion) {
    c.bench_function("init_server", |b| b.iter(|| server::Server::new()));
}

criterion_group!(init_server_bench, init_server);
criterion_main!(init_server_bench);

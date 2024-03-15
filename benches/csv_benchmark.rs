use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simplecsv::parse_from_file;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parsefile", |b| {
        b.iter(|| parse_from_file("tests/files/organizations-100000.csv", true))
    });
    let file = parse_from_file("tests/files/organizations-100000.csv", true);
    let mut file = file.unwrap();
    c.bench_function("setfield", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let _ = file.set_value_by_index(i * 26, i * 33, format!("test {i}"));
            }
        })
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

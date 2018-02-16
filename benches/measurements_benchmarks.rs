#[macro_use]
extern crate criterion;
extern crate wise_units;

use criterion::Criterion;
use wise_units::Measurement;

fn create_meter(c: &mut Criterion) {
    c.bench_function("create meter", |b| b.iter(|| Measurement::new(1.0, "m")));
}

fn create_km(c: &mut Criterion) {
    c.bench_function("create km", |b| {
        b.iter(|| Measurement::new(1.0, "km"));
    });
}

fn create_km2(c: &mut Criterion) {
    c.bench_function("create km2", |b| {
        b.iter(|| Measurement::new(1.0, "km2"));
    });
}

fn create_m2_per_second(c: &mut Criterion) {
    c.bench_function("create m2/s", |b| {
        b.iter(|| Measurement::new(1.0, "m2/s"));
    });
}

fn create_km2_per_second(c: &mut Criterion) {
    c.bench_function("create km2/s", |b| {
        b.iter(|| Measurement::new(1.0, "km2/s"));
    });
}

fn create_10km2_per_100second(c: &mut Criterion) {
    c.bench_function("create 10km2/100s", |b| {
        b.iter(|| Measurement::new(1.0, "10km2/100s"));
    });
}

fn create_millimole_per_hour(c: &mut Criterion) {
    c.bench_function("create mmol/h", |b| {
        b.iter(|| Measurement::new(1.0, "mmol/h"));
    });
}

fn create_milliliter_per_kilogram_and_hour(c: &mut Criterion) {
    c.bench_function("create mL/(kg.h)", |b| {
        b.iter(|| Measurement::new(1.0, "mL/(kg.h)"));
    });
}

fn convert_meter_to_km(c: &mut Criterion) {
    let measurement = &Measurement::new(1.0, "m").unwrap();

    c.bench_function("convert m to km", |b| {
        b.iter(|| measurement.convert_to("km").unwrap());
    });
}

fn convert_km_to_meter(c: &mut Criterion) {
    let km = &Measurement::new(1.0, "km").unwrap();

    c.bench_function("convert km to m", |b| {
        b.iter(|| km.convert_to("m").unwrap())
    });
}

fn add_km_to_meter(c: &mut Criterion) {
    let km = &Measurement::new(1.0, "km").unwrap();
    let m = &Measurement::new(1.0, "m").unwrap();

    c.bench_function("add km to m", |b| b.iter(|| km + m));
}

fn sub_km_by_meter(c: &mut Criterion) {
    let km = &Measurement::new(1.0, "km").unwrap();
    let m = &Measurement::new(1.0, "m").unwrap();

    c.bench_function("subtract km by m", |b| b.iter(|| km - m));
}

fn mul_km_by_meter(c: &mut Criterion) {
    let km = &Measurement::new(1.0, "km").unwrap();
    let m = &Measurement::new(1.0, "m").unwrap();

    c.bench_function("multiply km by m", |b| b.iter(|| km * m));
}

fn div_km_by_m(c: &mut Criterion) {
    let km = &Measurement::new(1.0, "km").unwrap();
    let m = &Measurement::new(1.0, "m").unwrap();

    c.bench_function("divide km by m", |b| b.iter(|| km / m));
}

criterion_group!(
    measurement_benches,
    create_meter,
    create_km,
    create_km2,
    create_m2_per_second,
    create_km2_per_second,
    create_10km2_per_100second,
    create_millimole_per_hour,
    create_milliliter_per_kilogram_and_hour,
    convert_meter_to_km,
    convert_km_to_meter,
    add_km_to_meter,
    sub_km_by_meter,
    mul_km_by_meter,
    div_km_by_m
);
criterion_main!(measurement_benches);

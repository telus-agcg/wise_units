#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;
use wise_units_parser::parse;

// use std::collections::{BTreeMap, HashMap};

// #[derive(PartialOrd)]
// struct Atom {
//     primary_code: &'static str,
//     value: f64,
// }

// impl PartialEq for Atom {
//     fn eq(&self, other: &Self) -> bool {
//         self.primary_code == other.primary_code
//     }
// }

// impl Atom {
//     const fn new(primary_code: &'static str, value: f64) -> Self {
//         Self {
//             primary_code,
//             value,
//         }
//     }
// }

// static ATOMS: [(&'static str, Atom); 41] = [
//     ("m", Atom::new("m", 1.0)),
//     ("s", Atom::new("s", 1.0)),
//     ("g", Atom::new("g", 1.0)),
//     ("rad", Atom::new("rad", 1.0)),
//     ("K", Atom::new("K", 1.0)),
//     ("C", Atom::new("C", 1.0)),
//     ("cd", Atom::new("cd", 1.0)),
//     ("10*", Atom::new("10*", 1.0)),
//     ("10^", Atom::new("10^", 1.0)),
//     ("[pi]", Atom::new("[pi]", std::f64::consts::PI)),
//     ("%", Atom::new("%", 1.0)),
//     ("[ppth]", Atom::new("[ppth]", 1.0)),
//     ("[ppm]", Atom::new("[ppm]", 1.0)),
//     ("[ppb]", Atom::new("[ppb]", 1.0)),
//     ("[pptr]", Atom::new("[pptr]", 1.0)),
//     ("mol", Atom::new("mol", 1.0)),
//     ("sr", Atom::new("sr", 1.0)),
//     ("Hz", Atom::new("Hz", 1.0)),
//     ("N", Atom::new("N", 1.0)),
//     ("Pa", Atom::new("Pa", 1.0)),
//     ("J", Atom::new("J", 1.0)),
//     ("W", Atom::new("W", 1.0)),
//     ("A", Atom::new("A", 1.0)),
//     ("V", Atom::new("V", 1.0)),
//     ("F", Atom::new("F", 1.0)),
//     ("Ohm", Atom::new("Ohm", 1.0)),
//     ("S", Atom::new("S", 1.0)),
//     ("Wb", Atom::new("Wb", 1.0)),
//     ("Cel", Atom::new("Cel", 1.0)),
//     ("T", Atom::new("T", 1.0)),
//     ("H", Atom::new("H", 1.0)),
//     ("lm", Atom::new("lm", 1.0)),
//     ("lx", Atom::new("lx", 1.0)),
//     ("Bq", Atom::new("Bq", 1.0)),
//     ("Gy", Atom::new("Gy", 1.0)),
//     ("Sv", Atom::new("Sv", 1.0)),
//     ("gon", Atom::new("gon", 1.0)),
//     ("deg", Atom::new("deg", 1.0)),
//     ("'", Atom::new("'", 1.0)),
//     ("''", Atom::new("''", 1.0)),
//     ("l", Atom::new("l", 1.0)),
// ];

// macro_rules! build_thing {
//     ($method_name:ident, $type:ident) => {
//         fn $method_name() -> $type<&'static str, Atom> {
//             let mut map = $type::new();
//             map.insert("m", Atom::new("m", 1.0));
//             map.insert("s", Atom::new("s", 1.0));
//             map.insert("g", Atom::new("g", 1.0));
//             map.insert("rad", Atom::new("rad", 1.0));
//             map.insert("K", Atom::new("K", 1.0));
//             map.insert("C", Atom::new("C", 1.0));
//             map.insert("cd", Atom::new("cd", 1.0));

//             map.insert("10*", Atom::new("10*", 1.0));
//             map.insert("10^", Atom::new("10^", 1.0));
//             map.insert("[pi]", Atom::new("[pi]", std::f64::consts::PI));
//             map.insert("%", Atom::new("%", 1.0));
//             map.insert("[ppth]", Atom::new("[ppth]", 1.0));
//             map.insert("[ppm]", Atom::new("[ppm]", 1.0));
//             map.insert("[ppb]", Atom::new("[ppb]", 1.0));
//             map.insert("[pptr]", Atom::new("[pptr]", 1.0));

//             map.insert("mol", Atom::new("mol", 1.0));
//             map.insert("sr", Atom::new("sr", 1.0));
//             map.insert("Hz", Atom::new("Hz", 1.0));
//             map.insert("N", Atom::new("N", 1.0));
//             map.insert("Pa", Atom::new("Pa", 1.0));
//             map.insert("J", Atom::new("J", 1.0));
//             map.insert("W", Atom::new("W", 1.0));
//             map.insert("A", Atom::new("A", 1.0));
//             map.insert("V", Atom::new("V", 1.0));
//             map.insert("F", Atom::new("F", 1.0));
//             map.insert("Ohm", Atom::new("Ohm", 1.0));
//             map.insert("S", Atom::new("S", 1.0));
//             map.insert("Wb", Atom::new("Wb", 1.0));
//             map.insert("Cel", Atom::new("Cel", 1.0));
//             map.insert("T", Atom::new("T", 1.0));
//             map.insert("H", Atom::new("H", 1.0));
//             map.insert("lm", Atom::new("lm", 1.0));
//             map.insert("lx", Atom::new("lx", 1.0));
//             map.insert("Bq", Atom::new("Bq", 1.0));
//             map.insert("Gy", Atom::new("Gy", 1.0));
//             map.insert("Sv", Atom::new("Sv", 1.0));
//             map.insert("gon", Atom::new("gon", 1.0));

//             map.insert("deg", Atom::new("deg", 1.0));
//             map.insert("'", Atom::new("'", 1.0));
//             map.insert("''", Atom::new("''", 1.0));
//             map.insert("l", Atom::new("l", 1.0));

//             map
//         }
//     };
// }
// build_thing!(build_hashmap, HashMap);
// build_thing!(build_btreemap, BTreeMap);

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse: m", |b| b.iter(|| parse(black_box("m"))));
    c.bench_function("parse: magain", |b| b.iter(|| parse("m")));

    c.bench_function("parse: m2", |b| b.iter(|| parse(black_box("m2"))));
    c.bench_function("parse: 2m", |b| b.iter(|| parse(black_box("2m"))));
    c.bench_function("parse: 2m2", |b| b.iter(|| parse(black_box("2m2"))));
    c.bench_function("parse: 2m2{things}", |b| {
        b.iter(|| parse(black_box("2m2{things}")))
    });

    c.bench_function("parse: km", |b| b.iter(|| parse(black_box("km"))));
    c.bench_function("parse: km2", |b| b.iter(|| parse(black_box("km2"))));
    c.bench_function("parse: 2km", |b| b.iter(|| parse(black_box("2km"))));
    c.bench_function("parse: 2km2", |b| b.iter(|| parse(black_box("2km2"))));
    c.bench_function("parse: 2km2{things}", |b| {
        b.iter(|| parse(black_box("2km2{things}")))
    });

    c.bench_function("parse: g.m", |b| b.iter(|| parse(black_box("g.m"))));
    c.bench_function("parse: g2.m", |b| b.iter(|| parse(black_box("g2.m"))));
    c.bench_function("parse: 2g.m", |b| b.iter(|| parse(black_box("2g.m"))));
    c.bench_function("parse: 2g2.m", |b| b.iter(|| parse(black_box("2g2.m"))));

    // c.bench_function("map: hashmap", |b| {
    //     let map = build_hashmap();

    //     b.iter(|| map.get("l").unwrap())
    // });

    // c.bench_function("map: btreemap", |b| {
    //     let map = build_btreemap();

    //     b.iter(|| map.get("l"))
    // });

    // c.bench_function("map: array iter().find()", |b| {
    //     b.iter(|| ATOMS.iter().find(|(k, _a)| k == &"l"))
    // });

    // c.bench_function("map: array binary search", |b| {
    //     // let mut atoms = ATOMS.iter().cloned().collect();
    //     ATOMS.sort();

    //     b.iter(|| ATOMS.binary_search("l"))
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

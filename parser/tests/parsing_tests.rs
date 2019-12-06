use wise_units_parser::parse;

macro_rules! test_parsing {
    ($test_name:ident, $unit_string:expr) => {
        #[test]
        fn $test_name() {
            assert!(parse($unit_string).is_ok());
        }
    };
}

test_parsing!(test_m, "m");
test_parsing!(test_m2, "m2");
test_parsing!(test_2m, "2m");
test_parsing!(test_2m2, "2m2");
test_parsing!(test_2m2things, "2m2{things}");

test_parsing!(test_km, "km");
test_parsing!(test_km2, "km2");
test_parsing!(test_2km, "2km");
test_parsing!(test_2km2, "2km2");
test_parsing!(test_2km2things, "2km2{things}");

test_parsing!(test_g_dot_m, "g.m");
test_parsing!(test_g2_dot_m, "g2.m");
test_parsing!(test_2g_dot_m, "2g.m");
test_parsing!(test_2g2_dot_m, "2g2.m");
test_parsing!(test_2g2things_dot_m, "2g2{things}.m");

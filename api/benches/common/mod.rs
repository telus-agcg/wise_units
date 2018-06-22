pub static UNIT_STRINGS: [&str; 25] = [
    // The unity
    "1",
    // base, factor*base, factor*base^exponent
    "m",
    "10m",
    "10m3",
    // prefix*base, factor*prefix*base, factor*prefix*base^exponent
    "cm",
    "10cm",
    "10cm3",
    // derived, factor*derived, factor*derived^exponent
    // [in_i] is defined in terms of cm (the previous test)
    "[in_i]",
    "10[in_i]",
    "10[in_i]3",
    // derived, factor*derived, factor*derived^exponent again
    // [gal_us] is defined in terms of [in_i] (the previous test)
    "[gal_us]",
    "10[gal_us]",
    "10[gal_us]3",
    // derived, factor*derived, factor*derived^exponent
    // mol is just a number
    "mol",
    "10mol",
    "10mol3",
    // special_derived, factor*special_derived, factor*special_derived^exponent
    "Cel",
    "10Cel",
    "10Cel3",
    // base/base, factor*base/base, factor*base^exponent/base
    "m/s",
    "10m/s",
    "10m3/s",
    // base/factor*base, base/factor*base^exponent
    "m/5s",
    "m/5s2",
    // factor*base^exponent/factor*base^exponent
    "10m3/5s2",
];

pub static UNIT_PAIRS: [(&str, &str); 19] = [
    ("m", "m"),
    ("m", "cm"),
    ("m", "[in_i]"),
    ("m", "[gal_us]"),
    ("m", "mol"),
    ("m", "Cel"),
    ("m", "m/s"),
    ("m", "cm/s"),
    ("m", "[in_i]/s"),
    ("m", "[gal_us]/s"),
    ("m", "mol/s"),
    ("m", "Cel/s"),
    ("[gal_us]", "[in_i]3"),
    ("mol", "10*10"),
    ("mol", "[gal_us]"),
    ("Cel", "[degF]"),
    ("Cel", "[pH]"),
    ("10[in_i]3", "100[in_us]3"),
    ("10[in_i]3", "100[in_us]2"),
];

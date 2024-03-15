pub(crate) const PREFIX_SYMBOLS: [&str; 20] = [
    "Y",  // yotta
    "Z",  // zetta
    "E",  // exa
    "P",  // peta
    "T",  // tera
    "G",  // giga
    "M",  // mega
    "k",  // kilo
    "h",  // hecto
    "da", // deka
    "d",  // deci
    "c",  // centi
    "m",  // milli
    "u",  // micro
    "n",  // nano
    "p",  // pico
    "f",  // femto
    "a",  // atto
    "z",  // zepto
    "y",  // yocto
];

#[rustfmt::skip]
pub(crate) const METRIC_ATOM_SYMBOLS: [&str; 96] = [
    // base units
    "m",
    "s",
    "g",
    "rad",
    "K",
    "C",
    "cd",
    // derived metric units
    "mol",
    "sr",
    "Hz",
    "N",
    "Pa",
    "J",
    "W",
    "A",
    "V",
    "F",
    "Ohm",
    "S",
    "Wb",
    "Cel",
    "T",
    "H",
    "lm",
    "lx",
    "Bq",
    "Gy",
    "Sv",
    "l",
    "L",
    "ar",
    "t",
    "bar",
    "u",
    "eV",
    "pc",
    "[c]",
    "[h]",
    "[k]",
    "[eps_0]",
    "[mu_0]",
    "[e]",
    "[m_e]",
    "[m_p]",
    "[G]",
    "[g]",
    "[ly]",
    "gf",
    "Ky",
    "Gal",
    "dyn",
    "erg",
    "P",
    "Bi",
    "St",
    "Mx",
    "G",
    "Oe",
    "Gb",
    "sb",
    "Lmb",
    "ph",
    "Ci",
    "R",
    "RAD",
    "REM",
    "cal_[15]",
    "cal_[20]",
    "cal_m",
    "cal_IT",
    "cal_th",
    "cal",
    "tex",
    "m[H2O]",
    "m[Hg]",
    "eq",
    "osm",
    "g%",
    "kat",
    "U",
    "[iU]",
    "[IU]",
    "Np",
    "B",
    "B[SPL]",
    "B[V]",
    "B[mV]",
    "B[uV]",
    "B[10.nV]",
    "B[W]",
    "B[kW]",
    "st",
    "mho",
    "bit",
    "By",
    "Bd"
];

#[rustfmt::skip]
pub(crate) const NON_METRIC_ATOM_SYMBOLS: [&str; 213] = [
    "[m/s2/Hz^(1/2)]",
    "[anti'Xa'U]",
    "[Amb'a'1'U]",
    "[stone_av]",
    "[in_i'H2O]",
    "[scwt_av]",
    "[lcwt_av]",
    "[ston_av]",
    "[lton_av]",
    "[in_i'Hg]",
    "[CCID_50]",
    "[TCID_50]",
    "[lbf_av]",
    "[rch_us]",
    "[rlk_us]",
    "[fth_us]",
    "[fur_us]",
    "[acr_us]",
    "[srd_us]",
    "[smi_us]",
    "[mil_us]",
    "[fth_br]",
    "[nmi_br]",
    "[acr_br]",
    "[gal_us]",
    "[bbl_us]",
    "[gil_us]",
    "[foz_us]",
    "[fdr_us]",
    "[min_us]",
    "[crd_us]",
    "[gal_wi]",
    "[dqt_us]",
    "[dpt_us]",
    "[tbs_us]",
    "[tsp_us]",
    "[cup_us]",
    "[gal_br]",
    "[gil_br]",
    "[foz_br]",
    "[fdr_br]",
    "[min_br]",
    "[pwt_tr]",
    "[pnt_pr]",
    "[pca_pr]",
    "[cicero]",
    "[Btu_39]",
    "[Btu_59]",
    "[Btu_60]",
    "[Btu_IT]",
    "[Btu_th]",
    "[wood'U]",
    "[p'diop]",
    "%[slope]",
    "[mesh_i]",
    "[hnsf'U]",
    "[beth'U]",
    "[todd'U]",
    "[smgy'U]",
    "[bdsk'U]",
    "[mclg'U]",
    "[EID_50]",
    "[D'ag'U]",
    "[car_Au]",
    "[fth_i]",
    "[nmi_i]",
    "[sin_i]",
    "[sft_i]",
    "[syd_i]",
    "[cin_i]",
    "[cft_i]",
    "[cyd_i]",
    "[mil_i]",
    "[cml_i]",
    "[ft_us]",
    "[yd_us]",
    "[in_us]",
    "[rd_us]",
    "[ch_us]",
    "[lk_us]",
    "[mi_us]",
    "[in_br]",
    "[ft_br]",
    "[rd_br]",
    "[ch_br]",
    "[lk_br]",
    "[pc_br]",
    "[yd_br]",
    "[mi_br]",
    "[kn_br]",
    "[qt_us]",
    "[pt_us]",
    "[bu_us]",
    "[pk_us]",
    "[foz_m]",
    "[cup_m]",
    "[tsp_m]",
    "[tbs_m]",
    "[pk_br]",
    "[bu_br]",
    "[qt_br]",
    "[pt_br]",
    "[lb_av]",
    "[oz_av]",
    "[dr_av]",
    "[oz_tr]",
    "[lb_tr]",
    "[sc_ap]",
    "[dr_ap]",
    "[oz_ap]",
    "[lb_ap]",
    "[pouce]",
    "[ligne]",
    "[didot]",
    "[degRe]",
    "[Btu_m]",
    "[hp'_X]",
    "[hp'_C]",
    "[hp'_M]",
    "[hp'_Q]",
    "[arb'U]",
    "[USP'U]",
    "[GPL'U]",
    "[MPL'U]",
    "[APL'U]",
    "[dye'U]",
    "[knk'U]",
    "[car_m]",
    "[smoot]",
    "[ppth]",
    "[pptr]",
    "[in_i]",
    "[ft_i]",
    "[yd_i]",
    "[mi_i]",
    "[kn_i]",
    "[bf_i]",
    "[cr_i]",
    "[hd_i]",
    "[oz_m]",
    "[pied]",
    "[degF]",
    "[degR]",
    "[diop]",
    "[hp_X]",
    "[hp_C]",
    "[hp_M]",
    "[hp_Q]",
    "[kp_X]",
    "[kp_C]",
    "[kp_M]",
    "[kp_Q]",
    "[ka'U]",
    "[tb'U]",
    "[ppm]",
    "[ppb]",
    "[sct]",
    "[twp]",
    "[lne]",
    "[pnt]",
    "[pca]",
    "[Cal]",
    "[Btu]",
    "[den]",
    "[PRU]",
    "[drp]",
    "[MET]",
    "[HPF]",
    "[LPF]",
    "[PFU]",
    "[FFU]",
    "[CFU]",
    "[BAU]",
    "[PNU]",
    "[FEU]",
    "[ELU]",
    "[psi]",
    "[pi]",
    "bit_s",
    "mo_s",
    "mo_j",
    "mo_g",
    "[gr]",
    "[HP]",
    "[Ch]",
    "[pH]",
    "[IR]",
    "[AU]",
    "[Lf]",
    "[EU]",
    "circ",
    "10*",
    "10^",
    "deg",
    "min",
    "a_t",
    "a_j",
    "a_g",
    "atm",
    "[S]",
    "att",
    "sph",
    "''",
    "wk",
    "mo",
    "AU",
    "Ao",
    "%",
    "'",
    "h",
    "d",
    "a",
    "b",
];

/// Concat each of our arrays of atom symbols into one.
///
pub(crate) const fn all_atom_symbols() -> [&'static str; 309] {
    let mut output = [""; 309];

    let mut i = 0;

    while i < METRIC_ATOM_SYMBOLS.len() {
        output[i] = METRIC_ATOM_SYMBOLS[i];
        i += 1;
    }

    let mut j = 0;

    while j < NON_METRIC_ATOM_SYMBOLS.len() {
        output[i] = NON_METRIC_ATOM_SYMBOLS[j];
        i += 1;
        j += 1;
    }

    output
}

// Sorts all atoms, first by length (longest first), then for atoms of equal length, those are
// sorted alphabetically.
//
pub(crate) fn all_atom_symbols_sorted() -> [&'static str; 309] {
    let mut sorted = all_atom_symbols();

    sorted.sort_by(|a, b| match a.len().cmp(&b.len()) {
        std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
        std::cmp::Ordering::Equal => a.cmp(b),
        std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
    });

    sorted
}

// Sorts only metric atoms, first by length (longest first), then for atoms of equal length, those
// are sorted alphabetically.
//
pub(crate) fn metric_atom_symbols_sorted() -> [&'static str; 96] {
    let mut sorted = METRIC_ATOM_SYMBOLS;

    sorted.sort_by(|a, b| match a.len().cmp(&b.len()) {
        std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
        std::cmp::Ordering::Equal => a.cmp(b),
        std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
    });

    sorted
}

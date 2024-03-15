use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    sequence::pair,
    IResult,
};

use crate::{unit, Atom, Prefix};

#[derive(Debug, PartialEq)]
pub(super) enum SimpleUnit {
    PrefixAtom { prefix: Prefix, atom: Atom },
    Atom(Atom),
}

// Docs:
// The prefix is the longest leading substring that matches a valid prefix where the remainder is a
// valid metric unit atom. If no such prefix can be matched, the unit atom is without prefix and
// may be both metric or non-metric.[1–3: ISO 1000, 3; ISO 2955-1983, 3.7; ANSI X3.50-1986, 3.7
// (Rule No. 6).]
//
pub(super) fn parse(input: &[u8]) -> IResult<&[u8], SimpleUnit> {
    const SYMBOL_CHARS: &str =
        "!#$%&'()*,./0123456789:;<>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[]^_`abcdefghijklmnopqrstuvwxyz|";

    let (tail, symbol) = alt((tag("10*"), tag("10^"), is_a(SYMBOL_CHARS)))(input)?;

    let (symbol_tail, simple_unit) =
        alt((parse_prefixed_metric_atom, parse_any_atom_symbol))(symbol)?;

    if symbol_tail.is_empty() {
        Ok((tail, simple_unit))
    } else {
        parse_any_atom_symbol(symbol)
    }
}

#[allow(clippy::too_many_lines)]
fn parse_any_atom_symbol(input: &[u8]) -> IResult<&[u8], SimpleUnit> {
    fn parse_8_char_symbol(input: &[u8]) -> IResult<&[u8], &[u8]> {
        alt((
            alt((
                tag("%[slope]"),
                tag("B[10.nV]"),
                tag("[Btu_39]"),
                tag("[Btu_59]"),
                tag("[Btu_60]"),
                tag("[Btu_IT]"),
                tag("[Btu_th]"),
                tag("[D'ag'U]"),
                tag("[EID_50]"),
                tag("[acr_br]"),
                tag("[acr_us]"),
                tag("[bbl_us]"),
                tag("[bdsk'U]"),
                tag("[beth'U]"),
                tag("[car_Au]"),
                tag("[cicero]"),
                tag("[crd_us]"),
                tag("[cup_us]"),
                tag("[dpt_us]"),
                tag("[dqt_us]"),
                tag("[fdr_br]"),
            )),
            alt((
                tag("[fdr_us]"),
                tag("[foz_br]"),
                tag("[foz_us]"),
                tag("[fth_br]"),
                tag("[fth_us]"),
                tag("[fur_us]"),
                tag("[gal_br]"),
                tag("[gal_us]"),
                tag("[gal_wi]"),
                tag("[gil_br]"),
                tag("[gil_us]"),
                tag("[hnsf'U]"),
                tag("[lbf_av]"),
                tag("[mclg'U]"),
                tag("[mesh_i]"),
                tag("[mil_us]"),
                tag("[min_br]"),
                tag("[min_us]"),
                tag("[nmi_br]"),
                tag("[p'diop]"),
            )),
            tag("[pca_pr]"),
            tag("[pnt_pr]"),
            tag("[pwt_tr]"),
            tag("[rch_us]"),
            tag("[rlk_us]"),
            tag("[smgy'U]"),
            tag("[smi_us]"),
            tag("[srd_us]"),
            tag("[tbs_us]"),
            tag("[todd'U]"),
            tag("[tsp_us]"),
            tag("[wood'U]"),
            tag("cal_[15]"),
            tag("cal_[20]"),
        ))(input)
    }

    fn parse_7_char_symbol(input: &[u8]) -> IResult<&[u8], &[u8]> {
        alt((
            alt((
                tag("[APL'U]"),
                tag("[Btu_m]"),
                tag("[GPL'U]"),
                tag("[MPL'U]"),
                tag("[USP'U]"),
                tag("[arb'U]"),
                tag("[bu_br]"),
                tag("[bu_us]"),
                tag("[car_m]"),
                tag("[cft_i]"),
                tag("[ch_br]"),
                tag("[ch_us]"),
                tag("[cin_i]"),
                tag("[cml_i]"),
                tag("[cup_m]"),
                tag("[cyd_i]"),
                tag("[degRe]"),
                tag("[didot]"),
                tag("[dr_ap]"),
                tag("[dr_av]"),
                tag("[dye'U]"),
            )),
            alt((
                tag("[eps_0]"),
                tag("[foz_m]"),
                tag("[ft_br]"),
                tag("[ft_us]"),
                tag("[fth_i]"),
                tag("[hp'_C]"),
                tag("[hp'_M]"),
                tag("[hp'_Q]"),
                tag("[hp'_X]"),
                tag("[in_br]"),
                tag("[in_us]"),
                tag("[kn_br]"),
                tag("[knk'U]"),
                tag("[lb_ap]"),
                tag("[lb_av]"),
                tag("[lb_tr]"),
                tag("[ligne]"),
                tag("[lk_br]"),
                tag("[lk_us]"),
                tag("[mi_br]"),
                tag("[mi_us]"),
            )),
            alt((
                tag("[mil_i]"),
                tag("[nmi_i]"),
                tag("[oz_ap]"),
                tag("[oz_av]"),
                tag("[oz_tr]"),
                tag("[pc_br]"),
                tag("[pk_br]"),
                tag("[pk_us]"),
                tag("[pouce]"),
                tag("[pt_br]"),
                tag("[pt_us]"),
                tag("[qt_br]"),
                tag("[qt_us]"),
                tag("[rd_br]"),
                tag("[rd_us]"),
                tag("[sc_ap]"),
                tag("[sft_i]"),
                tag("[sin_i]"),
                tag("[smoot]"),
                tag("[syd_i]"),
                tag("[tbs_m]"),
            )),
            tag("[tsp_m]"),
            tag("[yd_br]"),
            tag("[yd_us]"),
        ))(input)
    }

    fn parse_6_char_symbol(input: &[u8]) -> IResult<&[u8], &[u8]> {
        alt((
            alt((
                tag("[bf_i]"),
                tag("[cr_i]"),
                tag("[degF]"),
                tag("[degR]"),
                tag("[diop]"),
                tag("[ft_i]"),
                tag("[hd_i]"),
                tag("[hp_C]"),
                tag("[hp_X]"),
                tag("[in_i]"),
                tag("[kn_i]"),
                tag("[mi_i]"),
                tag("[mu_0]"),
                tag("[oz_m]"),
                tag("[pied]"),
                tag("[ppth]"),
                tag("[pptr]"),
                tag("[yd_i]"),
                tag("cal_IT"),
                tag("cal_th"),
                tag("m[H2O]"),
            )),
            tag("B[SPL]"),
            tag("[hp_M]"),
            tag("[hp_Q]"),
            tag("[ka'U]"),
            tag("[kp_C]"),
            tag("[kp_M]"),
            tag("[kp_Q]"),
            tag("[kp_X]"),
            tag("[tb'U]"),
        ))(input)
    }

    fn parse_5_char_symbol(input: &[u8]) -> IResult<&[u8], &[u8]> {
        alt((
            alt((
                tag("[Btu]"),
                tag("[Cal]"),
                tag("[FFU]"),
                tag("[HPF]"),
                tag("[LPF]"),
                tag("[MET]"),
                tag("[PFU]"),
                tag("[PRU]"),
                tag("[den]"),
                tag("[drp]"),
                tag("[lne]"),
                tag("[m_e]"),
                tag("[m_p]"),
                tag("[pca]"),
                tag("[pnt]"),
                tag("[ppb]"),
                tag("[ppm]"),
                tag("[sct]"),
                tag("[twp]"),
                tag("cal_m"),
                tag("m[Hg]"),
            )),
            tag("B[kW]"),
            tag("B[mV]"),
            tag("B[uV]"),
            tag("[BAU]"),
            tag("[CFU]"),
            tag("[ELU]"),
            tag("[FEU]"),
            tag("[PNU]"),
            tag("[psi]"),
            tag("bit_s"),
        ))(input)
    }

    fn parse_4_char_symbol(input: &[u8]) -> IResult<&[u8], &[u8]> {
        alt((
            tag("B[V]"),
            tag("B[W]"),
            tag("[AU]"),
            tag("[Ch]"),
            tag("[EU]"),
            tag("[HP]"),
            tag("[IR]"),
            tag("[IU]"),
            tag("[Lf]"),
            tag("[gr]"),
            tag("[iU]"),
            tag("[ly]"),
            tag("[pH]"),
            tag("[pi]"),
            tag("circ"),
            tag("mo_g"),
            tag("mo_j"),
            tag("mo_s"),
        ))(input)
    }

    fn parse_3_char_symbol(input: &[u8]) -> IResult<&[u8], &[u8]> {
        alt((
            alt((
                tag("10*"),
                tag("10^"),
                tag("Cel"),
                tag("Gal"),
                tag("Lmb"),
                tag("Ohm"),
                tag("RAD"),
                tag("REM"),
                tag("[G]"),
                tag("[S]"),
                tag("[c]"),
                tag("[e]"),
                tag("[g]"),
                tag("[h]"),
                tag("[k]"),
                tag("a_g"),
                tag("a_j"),
                tag("a_t"),
                tag("atm"),
                tag("att"),
                tag("bar"),
            )),
            tag("bit"),
            tag("cal"),
            tag("deg"),
            tag("dyn"),
            tag("erg"),
            tag("gon"),
            tag("kat"),
            tag("mho"),
            tag("min"),
            tag("mol"),
            tag("osm"),
            tag("rad"),
            tag("sph"),
            tag("tex"),
        ))(input)
    }

    fn parse_2_char_symbol(input: &[u8]) -> IResult<&[u8], &[u8]> {
        alt((
            alt((
                tag("''"),
                tag("AU"),
                tag("Ao"),
                tag("Bd"),
                tag("Bi"),
                tag("Bq"),
                tag("By"),
                tag("Ci"),
                tag("Gb"),
                tag("Gy"),
                tag("Hz"),
                tag("Ky"),
                tag("Mx"),
                tag("Np"),
                tag("Oe"),
                tag("Pa"),
                tag("St"),
                tag("Sv"),
                tag("Wb"),
                tag("ar"),
                tag("cd"),
            )),
            tag("eV"),
            tag("eq"),
            tag("g%"),
            tag("gf"),
            tag("lm"),
            tag("lx"),
            tag("mo"),
            tag("pc"),
            tag("ph"),
            tag("sb"),
            tag("sr"),
            tag("st"),
            tag("wk"),
        ))(input)
    }

    fn parse_1_char_symbol(input: &[u8]) -> IResult<&[u8], &[u8]> {
        alt((
            alt((
                tag("%"),
                tag("'"),
                tag("A"),
                tag("B"),
                tag("C"),
                tag("F"),
                tag("G"),
                tag("H"),
                tag("J"),
                tag("K"),
                tag("L"),
                tag("N"),
                tag("P"),
                tag("R"),
                tag("S"),
                tag("T"),
                tag("U"),
                tag("V"),
                tag("W"),
                tag("a"),
                tag("b"),
            )),
            tag("d"),
            tag("g"),
            tag("h"),
            tag("l"),
            tag("m"),
            tag("s"),
            tag("t"),
            tag("u"),
        ))(input)
    }

    let (tail, atom_symbol) = alt((
        tag("[m/s2/Hz^(1/2)]"),
        alt((tag("[anti'Xa'U]"), tag("[Amb'a'1'U]"))),
        alt((tag("[stone_av]"), tag("[in_i'H2O]"))),
        alt((
            tag("[scwt_av]"),
            tag("[lcwt_av]"),
            tag("[ston_av]"),
            tag("[lton_av]"),
            tag("[in_i'Hg]"),
            tag("[CCID_50]"),
            tag("[TCID_50]"),
        )),
        parse_8_char_symbol,
        parse_7_char_symbol,
        parse_6_char_symbol,
        parse_5_char_symbol,
        parse_4_char_symbol,
        parse_3_char_symbol,
        parse_2_char_symbol,
        parse_1_char_symbol,
    ))(input)
    .map(|(tail, atom_bytes)| {
        let atom_str = std::str::from_utf8(atom_bytes).unwrap();
        (tail, atom_str)
    })?;

    let atom = unit::parser::simple_unit::atom_symbol_to_atom(atom_symbol);
    Ok((tail, SimpleUnit::Atom(atom)))
}

// NOTE: This only tries to match prefixed atoms (which can only be metric atoms); matching
// non-prefixed atoms (either metric or non-metric) is handled elsewhere.
//
// NOTE: We try parsing 2-character prefixes with all the `input` first, to avoid the `"dar"` case,
// where `d` and `da` are both prefixes, but `da` + `r` isn't a unit, but `d` + `ar` is.
//
fn parse_prefixed_metric_atom(input: &[u8]) -> IResult<&[u8], SimpleUnit> {
    fn parse_2_char_prefix_symbol(input: &[u8]) -> IResult<&[u8], Prefix> {
        let (tail, prefix_symbol) = tag("da")(input).map(|(tail, prefix_bytes)| {
            let prefix_str = std::str::from_utf8(prefix_bytes).unwrap();
            (tail, prefix_str)
        })?;

        Ok((
            tail,
            unit::parser::simple_unit::prefix_symbol_to_prefix(prefix_symbol),
        ))
    }

    fn parse_1_char_prefix_symbol(input: &[u8]) -> IResult<&[u8], Prefix> {
        let (tail, prefix_symbol) = alt((
            tag("E"),
            tag("G"),
            tag("M"),
            tag("P"),
            tag("T"),
            tag("Y"),
            tag("Z"),
            tag("a"),
            tag("c"),
            tag("d"),
            tag("f"),
            tag("h"),
            tag("k"),
            tag("m"),
            tag("n"),
            tag("p"),
            tag("u"),
            tag("y"),
            tag("z"),
        ))(input)
        .map(|(tail, prefix_bytes)| {
            let prefix_str = std::str::from_utf8(prefix_bytes).unwrap();
            (tail, prefix_str)
        })?;

        Ok((
            tail,
            unit::parser::simple_unit::prefix_symbol_to_prefix(prefix_symbol),
        ))
    }

    eprintln!(
        "parsing metric atom: {}",
        std::str::from_utf8(input).unwrap()
    );
    let first_parser = pair(parse_2_char_prefix_symbol, parse_metric_atom_symbol);
    let second_parser = pair(parse_1_char_prefix_symbol, parse_metric_atom_symbol);
    let (tail, (prefix, atom)) = alt((first_parser, second_parser))(input)?;

    Ok((tail, SimpleUnit::PrefixAtom { prefix, atom }))
}

#[allow(clippy::too_many_lines)]
fn parse_metric_atom_symbol(input: &[u8]) -> IResult<&[u8], Atom> {
    let (tail, symbol) = alt((
        alt((tag("B[10.nV]"), tag("cal_[15]"), tag("cal_[20]"))),
        tag("[eps_0]"),
        alt((
            tag("B[SPL]"),
            tag("[mu_0]"),
            tag("cal_IT"),
            tag("cal_th"),
            tag("m[H2O]"),
        )),
        alt((
            tag("B[kW]"),
            tag("B[mV]"),
            tag("B[uV]"),
            tag("[m_e]"),
            tag("[m_p]"),
            tag("cal_m"),
            tag("m[Hg]"),
        )),
        alt((
            tag("B[V]"),
            tag("B[W]"),
            tag("[IU]"),
            tag("[iU]"),
            tag("[ly]"),
        )),
        alt((
            tag("Cel"),
            tag("Gal"),
            tag("Lmb"),
            tag("Ohm"),
            tag("RAD"),
            tag("REM"),
            tag("[G]"),
            tag("[c]"),
            tag("[e]"),
            tag("[g]"),
            tag("[h]"),
            tag("[k]"),
            tag("bar"),
            tag("bit"),
            tag("cal"),
            tag("dyn"),
            tag("erg"),
            tag("kat"),
            tag("mho"),
            tag("mol"),
            tag("osm"),
        )),
        alt((tag("rad"), tag("tex"))),
        alt((
            tag("Bd"),
            tag("Bi"),
            tag("Bq"),
            tag("By"),
            tag("Ci"),
            tag("Gb"),
            tag("Gy"),
            tag("Hz"),
            tag("Ky"),
            tag("Mx"),
            tag("Np"),
            tag("Oe"),
            tag("Pa"),
            tag("St"),
            tag("Sv"),
            tag("Wb"),
            tag("ar"),
            tag("cd"),
            tag("eV"),
            tag("eq"),
            tag("g%"),
        )),
        alt((
            tag("gf"),
            tag("lm"),
            tag("lx"),
            tag("pc"),
            tag("ph"),
            tag("sb"),
            tag("sr"),
            tag("st"),
        )),
        alt((
            tag("A"),
            tag("B"),
            tag("C"),
            tag("F"),
            tag("G"),
            tag("H"),
            tag("J"),
            tag("K"),
            tag("L"),
            tag("N"),
            tag("P"),
            tag("R"),
            tag("S"),
            tag("T"),
            tag("U"),
            tag("V"),
            tag("W"),
            tag("g"),
            tag("l"),
            tag("m"),
            tag("s"),
        )),
        tag("t"),
        tag("u"),
    ))(input)
    .map(|(tail, atom_bytes)| {
        let atom_str = std::str::from_utf8(atom_bytes).unwrap();
        (tail, atom_str)
    })?;

    Ok((tail, unit::parser::simple_unit::atom_symbol_to_atom(symbol)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefixed_metric_atom_symbol_test() {
        let sorted = unit::testing::metric_atom_symbols_sorted();

        for atom_symbol in &sorted {
            for prefix_symbol in &unit::testing::PREFIX_SYMBOLS {
                let unit = format!("{prefix_symbol}{atom_symbol}");

                let prefix = unit::parser::simple_unit::prefix_symbol_to_prefix(prefix_symbol);
                let atom = unit::parser::simple_unit::atom_symbol_to_atom(atom_symbol);

                validate_parse!(
                    unit_str: unit,
                    parser: parse_prefixed_metric_atom,
                    expected: SimpleUnit::PrefixAtom {
                        prefix,
                        atom
                    }
                );
                validate_parse!(
                    unit_str: unit,
                    parser: parse,
                    expected: SimpleUnit::PrefixAtom {
                        prefix,
                        atom
                    }
                );
            }
        }
    }
}

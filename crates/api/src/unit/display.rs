use std::{borrow::Cow, fmt};

use crate::{Term, Unit};

//-----------------------------------------------------------------------------
// impl Display
//-----------------------------------------------------------------------------
impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (numerators, denominators) = decompose(&self.terms);

        write!(f, "{}", recompose(&numerators, &denominators))
    }
}

/// Turns `terms` into two groups of strings: one for numerator terms, one for denominator terms.
/// These just need to be properly jointed by dots and slashes.
///
fn decompose(terms: &[Term]) -> (Vec<Cow<'_, str>>, Vec<String>) {
    let mut numerators = Vec::new();
    let mut denominators = Vec::new();

    for term in terms {
        match term.exponent() {
            Some(exponent) if exponent.is_positive() => {
                numerators.push(term.as_cow_str());
            }
            Some(exponent) => {
                let mut positive_exponent_term = term.clone();

                denominators.push(
                    positive_exponent_term
                        .set_exponent(exponent.abs())
                        .to_string(),
                );
            }
            None => numerators.push(term.as_cow_str()),
        }
    }

    (numerators, denominators)
}

fn recompose<'a>(numerators: &[Cow<'a, str>], denominators: &[String]) -> Cow<'a, str> {
    match (numerators.len(), denominators.len()) {
        (0, 0) => Cow::Borrowed("1"),
        (0, _) => Cow::Owned(format!("/{}", denominators.join("."))),
        (1, 0) => numerators[0].clone(),
        (_, 0) => Cow::Owned(numerators.join(".")),
        (_, _) => Cow::Owned(format!(
            "{}/{}",
            numerators.join("."),
            denominators.join("."),
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::{testing::const_units::l1::METER, unit::UNITY};

    macro_rules! test_display {
        ($test_name:ident: $unit:expr => $expected:expr) => {
            #[test]
            fn $test_name() {
                assert_eq!($unit.to_string(), $expected);
            }
        };
    }

    // ╭───────────────────╮
    // │ Single-term units │
    // ╰───────────────────╯
    test_display!(test_annotation: parse_unit!("{stuff}") => "{stuff}");
    test_display!(test_per_annotation: parse_unit!("/{stuff}") => "/{stuff}");

    test_display!(test_atom: METER => "m");
    test_display!(test_per_atom: parse_unit!("/m") => "/m");
    test_display!(test_atom_positive_1_exponent: parse_unit!("m+1") => "m");

    test_display!(test_unity: UNITY => "1");
    test_display!(test_per_unity: parse_unit!("/1") => "/1");

    test_display!(test_atom_annotation: parse_unit!("m{stuff}") => "m{stuff}");
    test_display!(test_atom_positive_1_exponent_annotation: parse_unit!("m+1{stuff}") => "m{stuff}");
    test_display!(test_per_atom_annotation: parse_unit!("/m{stuff}") => "/m{stuff}");

    test_display!(test_atom_exponent: parse_unit!("m2") => "m2");
    test_display!(test_per_atom_exponent: parse_unit!("/m2") => "/m2");
    test_display!(test_atom_positive_exponent: parse_unit!("m+2") => "m2");
    test_display!(test_atom_negative_exponent: parse_unit!("m-2") => "/m2");

    test_display!(test_atom_exponent_annotation: parse_unit!("m2{stuff}") => "m2{stuff}");
    test_display!(test_per_atom_exponent_annotation: parse_unit!("/m2{stuff}") => "/m2{stuff}");
    test_display!(test_atom_positive_exponent_annotation: parse_unit!("m+2{stuff}") => "m2{stuff}");
    test_display!(test_atom_negative_exponent_annotation: parse_unit!("m-2{stuff}") => "/m2{stuff}");

    // Prefix+
    test_display!(test_prefix_atom: parse_unit!("km") => "km");
    test_display!(test_per_prefix_atom: parse_unit!("/km") => "/km");
    test_display!(test_prefix_atom_positive_1_exponent: parse_unit!("km+1") => "km");
    test_display!(test_prefix_atom_negative_1_exponent: parse_unit!("km-1") => "/km");

    test_display!(test_prefix_atom_annotation: parse_unit!("km{stuff}") => "km{stuff}");
    test_display!(test_per_prefix_atom_annotation: parse_unit!("/km{stuff}") => "/km{stuff}");
    test_display!(test_per_prefix_atom_positive_1_exponent_annotation: parse_unit!("km+1{stuff}") => "km{stuff}");
    test_display!(test_per_prefix_atom_negative_1_exponent_annotation: parse_unit!("km-1{stuff}") => "/km{stuff}");

    test_display!(test_prefix_atom_exponent: parse_unit!("km2") => "km2");
    test_display!(test_per_prefix_atom_exponent: parse_unit!("/km2") => "/km2");
    test_display!(test_prefix_atom_negative_exponent: parse_unit!("km-2") => "/km2");

    test_display!(test_prefix_atom_exponent_annotation:
        parse_unit!("km2{stuff}") => "km2{stuff}");
    test_display!(test_per_prefix_atom_exponent_annotation:
        parse_unit!("/km2{stuff}") => "/km2{stuff}");
    test_display!(test_prefix_atom_positive_exponent_annotation:
        parse_unit!("km+2{stuff}") => "km2{stuff}");
    test_display!(test_prefix_atom_negative_exponent_annotation:
        parse_unit!("km-2{stuff}") => "/km2{stuff}");

    // Factor+
    test_display!(test_factor: parse_unit!("42") => "42");
    test_display!(test_per_factor: parse_unit!("/42") => "/42");

    test_display!(test_factor_annotation: parse_unit!("42{stuff}") => "42{stuff}");
    test_display!(test_per_factor_annotation: parse_unit!("/42{stuff}") => "/42{stuff}");

    test_display!(test_factor_exponent: unit!(term!(factor: 42, exponent: 2)) => "42+2");
    test_display!(test_per_factor_exponent: unit!(term!(factor: 42, exponent: -2)) => "/42+2");

    test_display!(test_factor_exponent_annotation:
        unit!(term!(factor: 42, exponent: 2, annotation: "stuff")) => "42+2{stuff}");
    test_display!(test_per_factor_exponent_annotation:
        unit!(term!(factor: 42, exponent: -2, annotation: "stuff")) => "/42+2{stuff}");

    test_display!(test_factor_atom: parse_unit!("10m") => "10m");
    test_display!(test_per_factor_atom: parse_unit!("/10m") => "/10m");

    test_display!(test_factor_atom_annotation: parse_unit!("10m{stuff}") => "10m{stuff}");
    test_display!(test_per_factor_atom_annotation: parse_unit!("/10m{stuff}") => "/10m{stuff}");

    test_display!(test_factor_atom_exponent: parse_unit!("10m2") => "10m2");
    test_display!(test_per_factor_atom_exponent: parse_unit!("/10m2") => "/10m2");

    test_display!(test_factor_atom_exponent_annotation:
        parse_unit!("10m2{stuff}") => "10m2{stuff}");
    test_display!(test_per_factor_atom_exponent_annotation:
        parse_unit!("/10m2{stuff}") => "/10m2{stuff}");

    test_display!(test_factor_prefix_atom: parse_unit!("10km") => "10km");
    test_display!(test_per_factor_prefix_atom: parse_unit!("/10km") => "/10km");

    test_display!(test_factor_prefix_atom_annotation: parse_unit!("10km{stuff}") => "10km{stuff}");
    test_display!(test_per_factor_prefix_atom_annotation:
        parse_unit!("/10km{stuff}") => "/10km{stuff}");

    test_display!(test_factor_prefix_atom_exponent:
        parse_unit!("10km3") => "10km3");
    test_display!(test_per_factor_prefix_atom_exponent:
        parse_unit!("/10km3") => "/10km3");

    test_display!(test_factor_prefix_atom_exponent_annotation:
        parse_unit!("10km3{stuff}") => "10km3{stuff}");
    test_display!(test_per_factor_prefix_atom_exponent_annotation:
        parse_unit!("/10km3{stuff}") => "/10km3{stuff}");
    test_display!(test_factor_prefix_atom_negative_exponent_annotation:
        parse_unit!("10km-3{stuff}") => "/10km3{stuff}");

    // ╭────────────────╮
    // │ Two-term units │
    // ╰────────────────╯
    // Notice that reduction doesn't happen here.
    test_display!(test_atom_exponent2_per_atom_exponent1: parse_unit!("m2/m") => "m2/m");
    test_display!(test_atom_exponent2_dot_atom_exponent_neg1: parse_unit!("m2.m-1") => "m2/m");

    test_display!(test_atom_per_other_atom_exponent: parse_unit!("m/s2") => "m/s2");
    test_display!(test_atom_dot_other_atom_negative_exponent: parse_unit!("m.s-2") => "m/s2");

    test_display!(test_atom_per_factor_atom: parse_unit!("m/10m") => "m/10m");
    test_display!(test_prefix_atom_per_factor_atom: parse_unit!("km/10m") => "km/10m");
    test_display!(test_prefix_atom_negative_exponent_per_atom_exponent: parse_unit!("km-1/m2") => "/km.m2");
    test_display!(test_prefix_atom_exponent_per_prefix_atom_exponent: parse_unit!("km3/nm2") => "km3/nm2");
    test_display!(test_paea_per_paea: parse_unit!("km3{foo}/nm2{bar}") => "km3{foo}/nm2{bar}");
    test_display!(test_annotation_per_same_annotation: parse_unit!("{foo}/{foo}") => "{foo}/{foo}");
    test_display!(test_annotation_per_different_annotation: parse_unit!("{foo}/{bar}") => "{foo}/{bar}");

    // ╭──────────────────╮
    // │ Three-term units │
    // ╰──────────────────╯
    test_display!(test_pa_per_ae_dot_pa: parse_unit!("km/m2.cm") => "km/m2.cm");
    test_display!(test_pa_per_anegativee_dot_pa: parse_unit!("km/m-2.cm") => "km.m2/cm");
    test_display!(test_pa_negative_1_per_ae_dot_pa: parse_unit!("km-1/m2.cm") => "/km.m2.cm");
}

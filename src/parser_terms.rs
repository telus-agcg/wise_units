use atom::Atom;
use prefix::Prefix;

#[derive(Clone, Debug, PartialEq)]
pub enum UnitSign {
    Positive,
    Negative
}

#[derive(Clone, Debug, PartialEq)]
pub struct Factor(pub u32);

#[derive(Clone, Debug, PartialEq)]
pub struct Exponent(pub UnitSign, pub u32);

#[derive(Clone, Debug, PartialEq)]
pub enum SimpleUnit {
    Atom(Atom),
    PrefixedAtom(Prefix, Atom),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Annotatable {
    Unit(SimpleUnit),
    UnitWithPower(SimpleUnit, Exponent),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Annotation<'a>(pub &'a str);

#[derive(Clone, Debug, PartialEq)]
pub enum Term<'a> {
    DotCombined(Component<'a>, Box<Term<'a>>),
    SlashCombined(Component<'a>, Box<Term<'a>>),
    Basic(Component<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Component<'a> {
    AnnotatedAnnotatable(Annotatable, Annotation<'a>),
    Annotatable(Annotatable),
    Annotation(Annotation<'a>),
    Factor(Factor),
    Term(Box<Term<'a>>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser::*;
    use atom::*;
    use prefix::*;

    #[test]
    fn validate_Exp() {
        assert_eq!(parse_Exponent("123").unwrap(), Exponent(UnitSign::Positive, 123));
        assert_eq!(parse_Exponent("-123").unwrap(), Exponent(UnitSign::Negative, 123));
    }

    #[test]
    fn validate_PrefixSymbol() {
        assert_eq!(parse_PrefixSymbol("k").unwrap(), PREFIXES[7]);
        assert_eq!(parse_PrefixSymbol("K").unwrap(), PREFIXES[7]);
    }

    #[test]
    fn validate_AtomSymbol() {
        assert_eq!(parse_AtomSymbol("m").unwrap(), ATOMS[0]);
        assert_eq!(parse_AtomSymbol("M").unwrap(), ATOMS[0]);
    }

    #[test]
    fn validate_SimpleUnit() {
        let su_atom = SimpleUnit::Atom(ATOMS[0].clone());
        let su_pre_atom = SimpleUnit::PrefixedAtom(PREFIXES[7].clone(), ATOMS[0].clone());

        assert_eq!(&parse_SimpleUnit("m").unwrap(), &su_atom);
        assert_eq!(&parse_SimpleUnit("km").unwrap(), &su_pre_atom);
        assert_eq!(&parse_SimpleUnit("M").unwrap(), &su_atom);
        assert_eq!(&parse_SimpleUnit("kM").unwrap(), &su_pre_atom);
        assert_eq!(&parse_SimpleUnit("Km").unwrap(), &su_pre_atom);
        assert_eq!(&parse_SimpleUnit("KM").unwrap(), &su_pre_atom);
    }

    #[test]
    fn validate_Annotatable() {
        let su_pre_atom = SimpleUnit::PrefixedAtom(PREFIXES[7].clone(), ATOMS[0].clone());
        let ann = Annotatable::Unit(su_pre_atom.clone());

        let ann_with_pos_power = Annotatable::UnitWithPower(
            su_pre_atom.clone(),
            Exponent(UnitSign::Positive, 10)
            );

        let ann_with_neg_power = Annotatable::UnitWithPower(
            su_pre_atom.clone(),
            Exponent(UnitSign::Negative, 10)
            );
        assert_eq!(&parse_Annotatable("km").unwrap(), &ann);
        assert_eq!(&parse_Annotatable("km10").unwrap(), &ann_with_pos_power);
        assert_eq!(&parse_Annotatable("km-10").unwrap(), &ann_with_neg_power);
    }

    #[test]
    fn validate_Annotation() {
        assert_eq!(parse_Annotation("{things123}").unwrap(), Annotation("things123"));
    }

    #[test]
    fn validate_Component_with_annotations() {
        let simple_unit = SimpleUnit::PrefixedAtom(PREFIXES[7].clone(), ATOMS[0].clone());
        let negative_exp = Exponent(UnitSign::Negative, 10);
        let annotatable = Annotatable::UnitWithPower(simple_unit, negative_exp);
        let annotation = Annotation("%vol");

        assert_eq!(
            parse_Component("km-10{%vol}").unwrap(),
            Component::AnnotatedAnnotatable(annotatable.clone(), annotation.clone())
            );

        assert_eq!(
            parse_Component("km-10").unwrap(),
            Component::Annotatable(annotatable.clone())
            );

        let annotation = Annotation("wet tis.");

        assert_eq!(
            parse_Component("{wet tis.}").unwrap(),
            Component::Annotation(annotation.clone())
            );
    }

    #[test]
    fn validate_Component_with_factor() {
        assert_eq!(
            parse_Component("123").unwrap(),
            Component::Factor(Factor(123))
            );
    }

    #[test]
    fn validate_Component_with_term() {
        assert_eq!(
            parse_Component("(123)").unwrap(),
            Component::Term(Box::new(Term::Basic(Component::Factor(Factor(123)))))
            );
    }

    #[test]
    fn validate_Term_with_dot() {
        assert_eq!(
            parse_Term("g.m").unwrap(),
            Term::DotCombined(
                Component::Annotatable(
                    Annotatable::Unit(
                        SimpleUnit::Atom(ATOMS[2].clone())
                    )
                ),
                Box::new(
                    Term::Basic(
                        Component::Annotatable(
                            Annotatable::Unit(
                                SimpleUnit::Atom(ATOMS[0].clone())
                            )
                        )
                    )
                )
            )
        );
    }

    #[test]
    fn validate_Term_with_slash() {
        assert_eq!(
            parse_Term("kg/s").unwrap(),
            Term::SlashCombined(
                Component::Annotatable(
                    Annotatable::Unit(
                        SimpleUnit::PrefixedAtom(PREFIXES[7].clone(), ATOMS[2].clone())
                    )
                ),
                Box::new(
                    Term::Basic(
                        Component::Annotatable(
                            Annotatable::Unit(
                                SimpleUnit::Atom(ATOMS[1].clone())
                            )
                        )
                    )
                )
            )
        );
    }

    #[test]
    fn validate_Term_basic() {
        assert_eq!(
            parse_Term("g").unwrap(),
            Term::Basic(
                Component::Annotatable(
                    Annotatable::Unit(
                        SimpleUnit::Atom(ATOMS[2].clone())
                    )
                )
            )
        );
    }
}

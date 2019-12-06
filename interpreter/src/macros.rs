macro_rules! build_runtime_unit {
    (
        $(
            $field_name:ident: $value:expr
        ),+ $(,)*
    )=> {
        {
            let mut unit = build_runtime_unit!();
            $(
                unit.$field_name = $value;
            )+
            unit
        }
    };

    () => {
        $crate::RuntimeUnit {
            factor: None,
            prefix: None,
            unit: None,
            annotation: None,
            exponent: 1,
            rhs: None,
        }
    };
}

use super::{atom, classification, mapper, property, symbol_grammar};
use handlebars::Handlebars;
use heck::CamelCase;

handlebars_helper!(camel_case_helper: |word: str| word.to_camel_case());

lazy_static! {
    pub static ref HANDLEBARS: Handlebars<'static> = {
        let mut handlebars = Handlebars::new();
        handlebars.register_escape_fn(::handlebars::no_escape);
        let _ = handlebars.register_helper("camelCase", Box::new(camel_case_helper));

        handlebars
            .register_template_string("atom", atom::HBS_TEMPLATE)
            .unwrap();
        handlebars
            .register_template_string("classification", classification::HBS_TEMPLATE)
            .unwrap();
        handlebars
            .register_template_string("property", property::HBS_TEMPLATE)
            .unwrap();
        handlebars
            .register_template_string("symbol_grammar", symbol_grammar::HBS_TEMPLATE)
            .unwrap();
        handlebars
            .register_template_string("mapper", mapper::HBS_TEMPLATE)
            .unwrap();

        handlebars
    };
}

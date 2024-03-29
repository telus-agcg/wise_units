use handlebars::{handlebars_helper, Handlebars};
use heck::ToUpperCamelCase;
use lazy_static::lazy_static;

use super::symbol_grammar;

handlebars_helper!(camel_case_helper: |word: str| word.to_upper_camel_case());

lazy_static! {
    pub static ref HANDLEBARS: Handlebars<'static> = {
        let mut handlebars = Handlebars::new();
        handlebars.register_escape_fn(::handlebars::no_escape);
        handlebars.register_helper("camelCase", Box::new(camel_case_helper));

        handlebars
            .register_template_string("symbol_grammar", symbol_grammar::HBS_TEMPLATE)
            .unwrap();

        handlebars
    };
}

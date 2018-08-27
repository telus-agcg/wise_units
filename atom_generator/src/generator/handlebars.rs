use handlebars::Handlebars;
use heck::CamelCase;

use super::atom;
use super::classification;
use super::mapper;
use super::property;
use super::symbol_grammar;
use super::symbol_parser;

lazy_static! {
    pub static ref HANDLEBARS: Handlebars = {
        let mut handlebars = Handlebars::new();
        handlebars.register_escape_fn(::handlebars::no_escape);
        handlebars.register_helper("camelCase", Box::new(camel_case_helper));

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
            .register_template_string("symbol_parser", symbol_parser::HBS_TEMPLATE)
            .unwrap();
        handlebars
            .register_template_string("mapper", mapper::HBS_TEMPLATE)
            .unwrap();

        handlebars
    };
}

handlebars_helper!(camel_case_helper: |word: str| word.to_camel_case());

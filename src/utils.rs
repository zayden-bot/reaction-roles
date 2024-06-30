use serenity::all::{ResolvedOption, ResolvedValue};
use std::collections::HashMap;

pub(crate) fn parse_options<'a>(
    options: &'a Vec<ResolvedOption<'_>>,
) -> HashMap<&'a str, &'a ResolvedValue<'a>> {
    let mut parsed_options = HashMap::new();

    for option in options {
        parsed_options.insert(option.name, &option.value);
    }

    parsed_options
}

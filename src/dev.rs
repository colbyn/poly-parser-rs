use tree_formatter::{PrettyTreePrinter, ToPrettyTree};
use colored::Colorize;

use crate::{combinators::SequenceSettings, data::{CharParser, ControlFlowParser, Parser, Text}};

pub fn dev() {
    let source = "Hello World";
    let parser = {
        fn is_whitespace() -> CharParser {
            CharParser::char_if(|x| x.is_whitespace())
        }
        let settings = SequenceSettings::default()
            .allow_empty(false)
            .until_terminator(ControlFlowParser::terminate_if_ok(is_whitespace));
        CharParser::next()
            .sequence(settings)
            .map(Text::from_iter)
    };
    let (output, state) = Parser::evaluate(source, parser);
    if let Some(output) = output {
        header("DONE");
        output.to_pretty_tree().print_pretty_tree();
    } else {
        header("ERROR!");
    }
    header("FINAL PARSER STATE");
    state.to_pretty_tree().print_pretty_tree();
}

pub fn header(value: impl AsRef<str>) {
    fn print_boxed_label(label: &str) {
        // - Calculate the length of the label -
        let length = label.chars().count();
        // - -
        let top_border    = format!("╭{}╮", "─".repeat(length + 2));
        let bottom_border = format!("╰{}╯", "─".repeat(length + 2));
        // - -
        let line1 = format!("{}", top_border).cyan();
        let line2 = format!("│ {} │", label).cyan();
        let line3 = format!("{}", bottom_border).cyan();
        // - -
        println!("{line1}");
        println!("{line2}");
        println!("{line3}");
    }
    print_boxed_label(value.as_ref())
}


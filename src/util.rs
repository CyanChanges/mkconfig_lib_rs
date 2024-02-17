#![allow(dead_code)]

use crate::misc::GRUB_TAB as Tab;
use crate::types::Statement;

pub trait Indent {
    fn indent_part(&self) -> String;
    fn indent(&self, append_new_line: bool) -> String;
}

impl Indent for &str {
    fn indent_part(&self) -> String {
        str_lines_indent(self, true)
    }
    fn indent(&self, append_new_line: bool) -> String {
        str_lines_indent(self, append_new_line)
    }
}

impl Indent for Vec<Statement> {
    fn indent_part(&self) -> String {
        statements_indent(self, true)
    }
    fn indent(&self, append_new_line: bool) -> String {
        statements_indent(self, append_new_line)
    }
}

#[macro_export]
macro_rules! indent {
    ($part: expr) => {
        $crate::util::Indent::indent_part($part)
    };
    ($part: expr, $append: expr) => {
        $crate::util::Indent::indent($part, $append)
    }
}

pub(crate) fn statements_indent(statement: &Vec<Statement>, append_newline: bool) -> String {
    if statement.is_empty() || (statement.iter().find(|x| !x.is_empty()).is_none()) {
        "".to_string()
    } else {
        Tab.to_string() + indent::indent_with(Tab, statement.join("\n")).as_str() + if append_newline { "\n" } else { "" }
    }
}

pub(crate) fn str_lines_indent(str: &str, append_new_line: bool) -> String {
    let statements = str.split('\n')
        .map(|x| x.to_string())
        .collect();

    statements_indent(&statements, append_new_line)
}

#[cfg(test)]
mod tests {
    use crate::misc::GRUB_TAB as Tab;
    use crate::util::statements_indent;

    #[test]
    fn test_indent() {
        assert_eq!( // With new line
            statements_indent(&vec![
                "Hello".to_string(),
                "World".to_string(),
            ], true),
                    format!("\
            {Tab}{}\n\
            {Tab}{}\n\
            ", "Hello", "World")
        );
        assert_eq!( // Without new line
            statements_indent(&vec![
                "Hello".to_string(),
                "World".to_string(),
            ], false),
                    format!("\
            {Tab}{}\n\
            {Tab}{}\
            ", "Hello", "World")
        );
    }

    #[test]
    fn test_indent_empty() {
        assert_eq!(
            statements_indent(&vec![], true),
            ""
        );
        assert_eq!(
            statements_indent(&vec!["".to_string()], true),
            ""
        );
        assert_eq!(
            statements_indent(&vec!["".to_string(), "".to_string()], true),
            ""
        );
    }
}

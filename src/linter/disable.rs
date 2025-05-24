use regex::{Captures, Regex};
use std::{collections::HashMap, sync::LazyLock};

/// Regex to match `// natlint-disable-next-line` directives
static NEXT_LINE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"//\s*natlint-disable-next-line(?:\s+([\w\s,]+))?").unwrap());

/// Inline directives to disable rules on a given file
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisableDirectives {
    rules_by_line: HashMap<usize, Option<Vec<String>>>,
}

impl DisableDirectives {
    const fn new(rules_by_line: HashMap<usize, Option<Vec<String>>>) -> Self {
        Self { rules_by_line }
    }

    pub fn is_disabled(&self, line: usize, rule_name: &str) -> bool {
        self.rules_by_line.get(&line).is_some_and(|rules| {
            rules
                .as_ref()
                .is_none_or(|disabled_rules| disabled_rules.contains(&rule_name.to_owned()))
        })
    }
}

/// Collect all `// natlint-disable-next-line …` directives.
///
/// * `key`   – line number **of the line that will be skipped** (1-based)
/// * `value` –
///   * `None`            → disable **all** rules on that line  
///   * `Some(Vec<…>)`    → disable only the listed rules
///
/// # Example
/// ```text
///  3 │ // natlint-disable-next-line
///  4 │ uint x;            // all rules disabled
///
///  8 │ // natlint-disable-next-line MissingNotice,MissingParam
///  9 │ function foo() {}  // only those two rules disabled
/// ```
pub fn disable_next_line_directives(content: &str) -> DisableDirectives {
    DisableDirectives::new(
        content
            .lines()
            .enumerate()
            .filter_map(|(idx, line)| {
                // Check if the line matches the regex
                NEXT_LINE_RE.captures(line).map(|caps| {
                    (
                        idx + 2, // The directive affects **next** line, so add 1 (and convert to 1-based)
                        rules_from_captures(&caps),
                    )
                })
            })
            .collect(),
    )
}

/// Extracts the rules from the regex captures.
/// If no rules are specified, returns `None`, which indicates that all rules should be disabled.
fn rules_from_captures(captures: &Captures) -> Option<Vec<String>> {
    let rules: Vec<String> = captures
        .get(1)?
        .as_str()
        .split(',')
        .map(|s| s.trim().to_owned())
        .filter(|s| !s.is_empty())
        .collect();

    (!rules.is_empty()).then_some(rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Macro to define a test case for `MissingParams` rule
    macro_rules! test_disable_next_line {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(
                    disable_next_line_directives($source),
                    DisableDirectives::new($expected)
                );
            }
        };
    }

    test_disable_next_line!(
        no_directives,
        r"
            uint a;
            function foo() {}
        ",
        HashMap::new()
    );

    test_disable_next_line!(
        disables_all_rules_on_next_line,
        r"
            // natlint-disable-next-line
            uint a;
        ",
        HashMap::from([(3, None)])
    );

    test_disable_next_line!(
        disables_single_rule,
        r"
            // natlint-disable-next-line MissingNotice
            uint a;
        ",
        HashMap::from([(3, Some(vec!["MissingNotice".to_owned()]))])
    );

    test_disable_next_line!(
        disables_multiple_rules_with_commas,
        r"
            // natlint-disable-next-line MissingNotice,MissingParam,AnotherRule
            uint a;
        ",
        HashMap::from([(
            3,
            Some(vec![
                "MissingNotice".to_owned(),
                "MissingParam".to_owned(),
                "AnotherRule".to_owned()
            ])
        )])
    );

    test_disable_next_line!(
        disables_multiple_rules_with_spaces,
        r"
            // natlint-disable-next-line MissingNotice, MissingParam, AnotherRule
            uint a;
        ",
        HashMap::from([(
            3,
            Some(vec![
                "MissingNotice".to_owned(),
                "MissingParam".to_owned(),
                "AnotherRule".to_owned()
            ])
        )])
    );

    test_disable_next_line!(
        multiple_directives_in_one_file,
        r"
            // natlint-disable-next-line
            uint a;

            // natlint-disable-next-line MissingNotice,MissingParam
            function foo() {}
        ",
        HashMap::from([
            (3, None),
            (
                6,
                Some(vec!["MissingNotice".to_owned(), "MissingParam".to_owned()])
            )
        ])
    );
}

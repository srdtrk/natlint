use regex::Regex;
use std::{collections::HashMap, sync::LazyLock};

/// `(?:\s+([\w\s,]+))?`
///   └──────┬───────── optional rule list, captured as group(1)
///          └───────── words / commas / (optional) whitespace
static RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"//\s*natlint-disable-next-line(?:\s+([\w\s,]+))?").unwrap());

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
#[allow(dead_code)]
pub fn disable_next_line_directives(content: &str) -> HashMap<usize, Option<Vec<String>>> {
    content
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            // Check if the line matches the regex
            RE.captures(line).map(|caps| {
                let entry = caps.get(1).map(|m| {
                    m.as_str()
                        .split(',')
                        .map(|s| s.trim().to_owned())
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<_>>()
                });

                (
                    idx + 2, // The directive affects **next** line, so add 1 (and convert to 1-based)
                    match entry {
                        Some(v) if v.is_empty() => None,
                        other => other,
                    },
                )
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Macro to define a test case for `MissingParams` rule
    macro_rules! test_disable_next_line {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(disable_next_line_directives($source), $expected);
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

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
///  8 │ // natlint-disable-next-line missing_notice,missing_params
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
    use std::collections::HashMap;

    /// Helper that builds a small HashMap in one line
    fn map(entry: (usize, Option<Vec<&str>>)) -> HashMap<usize, Option<Vec<String>>> {
        let (k, v) = entry;
        HashMap::from([(k, v.map(|vv| vv.into_iter().map(str::to_owned).collect()))])
    }

    #[test]
    fn disables_all_rules_on_next_line() {
        let src = "\
            // natlint-disable-next-line\n\
            uint a;\n";
        let expected = map((2, None));
        assert_eq!(disable_next_line_directives(src), expected);
    }

    #[test]
    fn disables_single_rule() {
        let src = "\
            // natlint-disable-next-line missing_notice\n\
            uint a;\n";
        let expected = map((2, Some(vec!["missing_notice"])));
        assert_eq!(disable_next_line_directives(src), expected);
    }

    #[test]
    fn disables_multiple_rules_with_spaces() {
        let src = "\
            // natlint-disable-next-line  missing_notice , missing_params ,another_rule\n\
            uint a;\n";
        let expected = map((
            2,
            Some(vec!["missing_notice", "missing_params", "another_rule"]),
        ));
        assert_eq!(disable_next_line_directives(src), expected);
    }

    #[test]
    fn multiple_directives_in_one_file() {
        let src = "\
            // natlint-disable-next-line\n\
            uint a;\n\
            \n\
            // natlint-disable-next-line rule_one,rule_two\n\
            function foo() {}\n";
        let mut expected = HashMap::new();
        expected.insert(2, None);
        expected.insert(5, Some(vec!["rule_one".to_owned(), "rule_two".to_owned()]));
        assert_eq!(disable_next_line_directives(src), expected);
    }
}

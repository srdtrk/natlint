use clap::Parser;
use line_col::LineColLookup;
use natlint::{
    cli::cmd::{Commands, NatlintCli},
    cli::file_finder::find_matching_files,
    parser::{Comments, CommentsRef, ParseItem, ParseSource},
    rules::{Rule, Violation},
};
use std::any::Any;
use std::sync::Arc;

/// A trait object that can check any parseable item
pub trait AnyRule: Send + Sync {
    /// Check if this rule applies to the given item
    fn applies_to(&self, item: &dyn Any) -> bool;
    
    /// Run the rule check on the given item if applicable
    fn check_item(&self, parent: Option<&ParseItem>, item: &dyn Any, comments: CommentsRef) -> Option<Violation>;
    
    /// Get the name of the rule
    fn name(&self) -> &'static str;
    
    /// Get the description of the rule
    fn description(&self) -> &'static str;
}

/// A wrapper to make Rule<T> implementors work with AnyRule
struct RuleWrapper<T: 'static + Send + Sync, R: Rule<T> + Send + Sync> {
    _phantom: std::marker::PhantomData<T>,
    _rule: std::marker::PhantomData<R>,
}

impl<T: 'static + Send + Sync, R: Rule<T> + Send + Sync> RuleWrapper<T, R> {
    /// Create a new rule wrapper
    fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
            _rule: std::marker::PhantomData,
        }
    }
}

impl<T: 'static + Send + Sync, R: Rule<T> + Send + Sync> AnyRule for RuleWrapper<T, R> {
    fn applies_to(&self, item: &dyn Any) -> bool {
        item.downcast_ref::<T>().is_some()
    }
    
    fn check_item(&self, parent: Option<&ParseItem>, item: &dyn Any, comments: CommentsRef) -> Option<Violation> {
        if let Some(concrete_item) = item.downcast_ref::<T>() {
            R::check(parent, concrete_item, comments)
        } else {
            None
        }
    }
    
    fn name(&self) -> &'static str {
        R::NAME
    }
    
    fn description(&self) -> &'static str {
        R::DESCRIPTION
    }
}

/// Configuration for natlint rules
pub struct Config {
    /// The collection of rules to apply
    rules: Vec<Arc<dyn AnyRule>>,
}

impl Config {
    /// Create a new empty configuration
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
        }
    }
    
    /// Add a rule to the configuration
    pub fn add_rule<T: 'static + Send + Sync, R: Rule<T> + Send + Sync + 'static>(&mut self) -> &mut Self {
        let rule = Arc::new(RuleWrapper::<T, R>::new());
        self.rules.push(rule);
        self
    }
    
    /// Check an item against all applicable rules
    pub fn check_item(&self, parent: Option<&ParseItem>, item: &dyn Any, comments: &Comments) -> Vec<Violation> {
        let mut violations = Vec::new();
        
        for rule in &self.rules {
            if rule.applies_to(item) {
                let comments_ref = CommentsRef::from(comments);
                if let Some(violation) = rule.check_item(parent, item, comments_ref) {
                    violations.push(violation);
                }
            }
        }
        
        violations
    }
}

/// Create a configuration with all available rules
fn load_default_config() -> Config {
    use natlint::rules::{
        contract::{
            self as contract_rules
        },
        function::{
            self as function_rules
        },
        r#struct::{
            self as struct_rules
        }
    };
    use solang_parser::pt::{ContractDefinition, FunctionDefinition, StructDefinition};

    let mut config = Config::new();
    
    // Contract Rules
    config.add_rule::<ContractDefinition, contract_rules::MissingAuthor>();
    config.add_rule::<ContractDefinition, contract_rules::MissingNotice>();
    config.add_rule::<ContractDefinition, contract_rules::MissingTitle>();
    config.add_rule::<ContractDefinition, contract_rules::NoInheritdoc>();
    config.add_rule::<ContractDefinition, contract_rules::NoParam>();
    config.add_rule::<ContractDefinition, contract_rules::NoReturn>();
    config.add_rule::<ContractDefinition, contract_rules::TooManyNotice>();
    config.add_rule::<ContractDefinition, contract_rules::TooManyTitle>();
    
    // Function Rules
    config.add_rule::<FunctionDefinition, function_rules::MissingInheritdoc>();
    config.add_rule::<FunctionDefinition, function_rules::MissingNotice>();
    config.add_rule::<FunctionDefinition, function_rules::MissingParams>();
    config.add_rule::<FunctionDefinition, function_rules::MissingReturn>();
    config.add_rule::<FunctionDefinition, function_rules::NoAuthor>();
    config.add_rule::<FunctionDefinition, function_rules::NoTitle>();
    config.add_rule::<FunctionDefinition, function_rules::OnlyInheritdoc>();
    
    // Struct Rules
    config.add_rule::<StructDefinition, struct_rules::MissingAuthor>();
    config.add_rule::<StructDefinition, struct_rules::MissingNotice>();
    config.add_rule::<StructDefinition, struct_rules::MissingParams>();
    config.add_rule::<StructDefinition, struct_rules::MissingTitle>();
    config.add_rule::<StructDefinition, struct_rules::NoInheritdoc>();
    config.add_rule::<StructDefinition, struct_rules::NoReturn>();
    config.add_rule::<StructDefinition, struct_rules::TooManyNotice>();
    config.add_rule::<StructDefinition, struct_rules::TooManyTitle>();
    
    config
}

/// Load configuration from a file or use defaults
fn load_config(config_path: &str) -> Config {
    // In a real implementation, this would parse TOML/YAML/JSON configuration
    // For now, we'll just use the default configuration
    if config_path.is_empty() {
        return load_default_config();
    }
    
    // TODO: Parse config file and selectively enable rules
    load_default_config()
}

/// Process a single Solidity file and return any violations with line numbers
fn process_file(file_path: &std::path::Path, config: &Config) -> eyre::Result<Vec<(String, Violation, usize)>> {
    use forge_fmt::Visitable;
    use solang_parser::parse;
    use std::fs;
    
    // Read file content
    let content = fs::read_to_string(file_path)?;
    
    // Create line/column lookup for efficient offset-to-line conversion
    let line_lookup = LineColLookup::new(&content);
    
    // Parse Solidity file
    let (mut source_unit, comments) = parse(&content, 0)
        .map_err(|e| eyre::eyre!("Failed to parse {}: {:?}", file_path.display(), e))?;
    
    // Create parser and visit the source unit
    let mut parser = natlint::parser::Parser::new(comments, content.clone());
    source_unit.visit(&mut parser)
        .map_err(|e| eyre::eyre!("Failed to visit {}: {:?}", file_path.display(), e))?;
    
    // Get parsed items
    let items = parser.items();
    
    // Collect violations
    let mut all_violations = Vec::new();
    
    // Function to recursively process items and their children
    fn process_item(
        item: &ParseItem,
        parent: Option<&ParseItem>,
        config: &Config,
        file_path: &std::path::Path,
        line_lookup: &LineColLookup,
        all_violations: &mut Vec<(String, Violation, usize)>,
    ) {
        // Check item against all applicable rules
        match &item.source {
            ParseSource::Contract(contract) => {
                let violations = config.check_item(
                    parent, 
                    &**contract as &dyn std::any::Any, 
                    &item.comments
                );
                
                for violation in violations {
                    // Get line number (1-based) using efficient lookup
                    let (line, _) = line_lookup.get(violation.loc.start());
                    all_violations.push((file_path.display().to_string(), violation, line));
                }
            },
            ParseSource::Function(function) => {
                let violations = config.check_item(
                    parent, 
                    function as &dyn std::any::Any, 
                    &item.comments
                );
                
                for violation in violations {
                    let (line, _) = line_lookup.get(violation.loc.start());
                    all_violations.push((file_path.display().to_string(), violation, line));
                }
            },
            ParseSource::Struct(structure) => {
                let violations = config.check_item(
                    parent, 
                    structure as &dyn std::any::Any, 
                    &item.comments
                );
                
                for violation in violations {
                    let (line, _) = line_lookup.get(violation.loc.start());
                    all_violations.push((file_path.display().to_string(), violation, line));
                }
            },
            // Add other item types as needed:
            // ParseSource::Enum(..) => { ... },
            // ParseSource::Error(..) => { ... },
            // ParseSource::Event(..) => { ... },
            // ParseSource::Variable(..) => { ... },
            // ParseSource::Type(..) => { ... },
            _ => {
                // No rules implemented for these item types yet
            }
        }
        
        // Process all children
        for child in &item.children {
            process_item(child, Some(item), config, file_path, line_lookup, all_violations);
        }
    }
    
    // Process all top-level items
    for item in &items {
        process_item(item, None, config, file_path, &line_lookup, &mut all_violations);
    }
    
    Ok(all_violations)
}

fn main() -> eyre::Result<()> {
    let cli = NatlintCli::parse();
    match cli.command {
        Commands::Run(args) => {
            println!("Running natlint with config: {}", args.config);
            
            // Display helpful message for using glob patterns if no include patterns are provided
            if args.include.is_empty() && args.root == "." {
                println!("Tip: Use --include/-i to specify glob patterns to search for files.");
                println!("Example: natlint run -c config.toml -i \"**/*.sol\" -e \"node_modules/**\"");
                println!("Searching for Solidity files in the current directory...")
            }

            // Load configuration with all rules
            let config = load_config(&args.config);

            // Find all files in the root directory that match the include globs and do not match the exclude globs
            let files = find_matching_files(&args.root, args.include, args.exclude)?;
            println!("Found {} files to lint", files.len());
            
            // Process each file and collect violations
            let mut all_violations = Vec::new();
            let mut error_count = 0;
            let total_files = files.len();
            
            // Show progress if there are more than 5 files
            let show_progress = total_files > 5;
            
            for (idx, file) in files.iter().enumerate() {
                // Show progress
                if show_progress && idx % 5 == 0 {
                    print!("\rProcessing files: {}/{} ({}%)", 
                        idx + 1, 
                        total_files, 
                        ((idx + 1) as f64 / total_files as f64 * 100.0) as u32
                    );
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                }
                
                match process_file(file, &config) {
                    Ok(violations) => {
                        all_violations.extend(violations);
                    },
                    Err(err) => {
                        if show_progress {
                            println!(); // New line after progress indicator
                        }
                        eprintln!("Error processing file {}: {}", file.display(), err);
                        error_count += 1;
                    }
                }
            }
            
            // Clear progress line when done
            if show_progress {
                println!("\rProcessed {} files                ", total_files);
            }
            
            // Sort violations by file, rule, and actual line number
            all_violations.sort_by(|(file_a, viol_a, line_a), (file_b, viol_b, line_b)| {
                file_a.cmp(file_b)
                    .then_with(|| line_a.cmp(line_b))
                    .then_with(|| viol_a.rule.cmp(viol_b.rule))
            });
            
            // Report violations
            if all_violations.is_empty() {
                println!("No natspec violations found!");
            } else {
                println!("\nNatspec violations found:");
                
                let mut current_file = String::new();
                let mut violation_count = 0;
                
                for (file, violation, line_number) in all_violations {
                    // Print file name when it changes
                    if current_file != file {
                        if !current_file.is_empty() {
                            println!();
                        }
                        println!("File: {}", file);
                        current_file = file;
                    }
                    
                    // Print violation details with converted line number
                    println!("  [{}] Line {}: {}", 
                        violation.rule,
                        line_number,
                        violation.description
                    );
                    
                    violation_count += 1;
                }
                
                println!("\nFound {} natspec violations in {} files.", violation_count, files.len());
                
                if error_count > 0 {
                    println!("Failed to process {} files due to errors.", error_count);
                }
                
                // Return non-zero exit code if violations were found
                std::process::exit(1);
            }
            
            // If there were parsing errors but no violations, still exit with error
            if error_count > 0 {
                println!("Failed to process {} files due to errors.", error_count);
                std::process::exit(1);
            }
            
            Ok(())
        }
    }
}

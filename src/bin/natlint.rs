use clap::Parser;
use natlint::{
    cli::cmd::{Commands, NatlintCli},
    cli::file_finder::find_matching_files,
    parser::{Comments, CommentsRef, ParseItem},
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

/// Load configuration from a file or use defaults
fn load_config(_config_path: &str) -> Config {
    // In a real implementation, this would load from a TOML/YAML/JSON file
    // For now, we'll just create a default config
    let config = Config::new();
    
    // Example of how rules would be added
    // config.add_rule::<solang_parser::pt::ContractDefinition, natlint::rules::contract::missing_author::MissingAuthor>();
    
    config
}

fn main() -> eyre::Result<()> {
    let cli = NatlintCli::parse();
    match cli.command {
        Commands::Run(args) => {
            println!("Running natlint with config: {}", args.config);

            // Load configuration
            let _config = load_config(&args.config);

            // Find all files in the root directory that match the include globs and do not match the exclude globs
            let files = find_matching_files(&args.root, args.include, args.exclude)?;
            files.iter().for_each(|file| println!("{}", file.display()));

            // TODO: Parse each file and apply rules using config.check_item()
            
            Ok(())
        }
    }
}

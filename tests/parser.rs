use quote::quote;
use std::{fs, path::Path};
use syn_solidity::{Expr, File, Item, Lit, Stmt};

#[test]
fn contracts() {
    static PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/contracts");
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let mut entries: Vec<_> = fs::read_dir(PATH)
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap();
    entries.sort_by_key(std::fs::DirEntry::path);
}

#[test]
fn parse_file() -> anyhow::Result<()> {
    // Create a Solidity `TokenStream`
    let tokens = quote! {
        /// @name HelloWorld
        /// @notice A hello world example in Solidity.
        contract HelloWorld {
            /// @notice Returns the string "Hello, World!".
            function helloWorld() external pure returns (string memory) {
                return "Hello, World!";
            }
        }
    };

    // Parse the tokens into a `File`
    let ast: File = syn_solidity::parse2(tokens)?;

    let items: &[Item] = &ast.items;
    let Some(Item::Contract(contract)) = items.first() else {
        unreachable!()
    };
    assert_eq!(contract.name, "HelloWorld");
    assert_eq!(contract.attrs.len(), 2); // doc comments

    let body: &[Item] = &contract.body;
    let Some(Item::Function(function)) = body.first() else {
        unreachable!()
    };
    assert_eq!(function.attrs.len(), 1); // doc comment
    assert_eq!(function.name.as_ref().unwrap(), "helloWorld");
    assert!(function.parameters.is_empty()); // ()
    assert_eq!(function.attributes.len(), 2); // external pure
    assert!(function.returns.is_some());

    let Some([Stmt::Return(ret)]) = function.body() else {
        unreachable!()
    };
    let Some(Expr::Lit(Lit::Str(s))) = &ret.expr else {
        unreachable!()
    };
    assert_eq!(s.value(), "Hello, World!");

    // Read doc comments
    // print doc comments
    println!("contract comments: {:?}", contract.attrs);
    println!("function comments: {:?}", function.attrs);

    Ok(())
}

//fn parse_file(path: &Path) -> Result<File, Box<dyn core::error::Error>> {
//    let solidity = fs::read_to_string(path)?;
//    syn::parse_str(&solidity).map_err(Into::into)
//}

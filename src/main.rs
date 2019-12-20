mod parser;

use std::io;
use parser::Parser;

fn main() {
    let parse_tree = Parser::parse_file(&mut io::stdin());
    match parse_tree {
        Ok(node) => match node.print(&mut io::stdout()) {
            Ok(_) => (),
            Err(e) => eprintln!("Failed to print parse result: {}", e)
        }
        Err(e) => eprintln!("Parse operation failed: {}", e)
    }
}

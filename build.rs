use std::io;
use std::{fs::File, io::Write, path::Path, process::exit};

fn main() {
    let args = std::env::var("RLOX_AST_DIR");
    if args.is_err() {
        eprintln!("Environment variable `RLOX_AST_DIR` is unset!");
        exit(64);
    }
    let output_dir = args.unwrap();
    define_ast(
        output_dir.as_str(),
        "Expr",
        vec![
            "Binary     : Expr left, Token operator, Expr right",
            "Grouping   : Expr expression",
            "Literal    : Object value",
            "Unary      : Token operator, Expr right",
        ],
    )
    .expect("Failed to build Expr ast file");
}

fn define_ast(output_dir: &str, base_name: &str, types: Vec<&'static str>) -> io::Result<()> {
    // check path
    let path = Path::new(output_dir);
    if !path.exists() {
        eprintln!("Output directory not exists!");
        exit(64);
    }
    let path = path.join(base_name.to_lowercase()).with_extension("rs");
    eprintln!("Output file path: {:?}", path);
    let mut output_file = File::create(path).expect("Failed to create output file.");
    writeln!(&mut output_file, "")?;

    Ok(())
}

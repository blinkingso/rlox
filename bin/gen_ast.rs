use std::{
    env,
    fs::{create_dir_all, File},
    io::{self, BufWriter},
    path::Path,
    process::exit,
};

fn main() {
    let args = env::args();
    if args.len() != 2 {
        eprintln!("Usage: gen_ast <output directory>");
        exit(64);
    }

    let output_dir = args.last().unwrap();
    println!("output_dir: {output_dir}");
    define_ast(
        output_dir.as_str(),
        "Expr",
        vec![
            "Binary    : Expr left, Token operator, Expr right",
            "Grouping  : Expr expression",
            "Literal   : Object value",
            "Unary     : Token operator, Expr right",
        ],
    );
}

fn define_ast(output_dir: &str, name: &str, types: Vec<&'static str>) {
    let output_dir = Path::new(output_dir);
    if !output_dir.exists() {
        create_dir_all(output_dir).expect(
            format!(
                "Unable to create output_dir: {}",
                output_dir.to_str().unwrap()
            )
            .as_str(),
        );
    }
    let full_path = output_dir.join(name.to_lowercase()).with_extension("rs");
    if !full_path.exists() {
        File::create(&full_path).expect(
            format!(
                "Unable to create file: {}",
                full_path.to_string_lossy().as_ref()
            )
            .as_str(),
        );
    }
    let mut _writer = BufWriter::new(io::stdout().lock());
}

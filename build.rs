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
    writeln!(&mut output_file, "use crate::token::*;")?;
    writeln!(&mut output_file, "use crate::literal::*;")?;
    writeln!(&mut output_file, " ")?;

    writeln!(&mut output_file, "pub enum {base_name} {{")?;

    for ty in types.iter() {
        let (class_name, _fields) = ty.split_once(":").unwrap();
        let class_name = class_name.trim();
        writeln!(&mut output_file, "\t{class_name}({class_name}{base_name}),")?;
    }

    writeln!(&mut output_file, "}}")?;

    for ty in types.iter() {
        let (class_name, fields) = ty.split_once(":").unwrap();
        define_type(
            &mut output_file,
            base_name,
            class_name.trim(),
            fields.trim(),
        )?;
    }

    // define visitor
    define_visitor(&mut output_file, base_name, types)?;

    Ok(())
}

fn define_visitor(writer: &mut File, base_name: &str, types: Vec<&'static str>) -> io::Result<()> {
    writeln!(writer, "trait Visitor<R> {{")?;
    for ty in types {
        let (ty_name, _) = ty.split_once(":").unwrap();
        let ty_name = ty_name.trim();
        writeln!(
            writer,
            "\tfn visit_{}_{}({}: Box<{}{}>) -> ::std::io::Result<R>;",
            ty_name.trim().to_lowercase(),
            base_name.to_lowercase(),
            base_name.to_lowercase(),
            ty_name,
            base_name,
        )?;
    }
    writeln!(writer, "}}")?;
    Ok(())
}

fn define_type(
    writer: &mut File,
    base_name: &str,
    class_name: &str,
    fields: &str,
) -> io::Result<()> {
    let ident_name = format!("{class_name}{base_name}");
    writeln!(writer, "pub struct {ident_name} {{")?;
    let fields = fields.split(",");
    for field in fields {
        let (ty, name) = field.rsplit_once(" ").unwrap();
        let ty = ty.trim();
        let name = name.trim();
        if ty.eq(base_name) {
            writeln!(writer, "\t{name}: Box<{ty}>,")?;
        } else {
            writeln!(writer, "\t{name}: {ty},")?;
        }
    }
    writeln!(writer, "}}")?;
    Ok(())
}

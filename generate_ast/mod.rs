// use std::fs::File;
// use std::io::{self, Write};

// #[derive(Debug)]
// struct TreeType {
//     base_class_name: String,
//     class_name: String,
//     fields: Vec<String>,
// }

// pub fn generate_ast(output_dir: String) -> io::Result<()> {
//     define_ast(
//         &output_dir.to_string(),
//         &"Expr".to_string(),
//         &vec![
//             "Binary   : Box<Expr> left, Token operator, Box<Expr> right".to_string(),
//             "Grouping : Box<Expr> expression".to_string(),
//             "Literal  : Object value".to_string(),
//             "Unary    : Token operator, Box<Expr> right".to_string(),
//         ],
//     )?;
//     Ok(())
// }

// fn define_ast(output_dir: &String, base_name: &String, types: &[String]) -> io::Result<()> {
//     println!("{}", base_name);
//     let path = format!("./{output_dir}/{}.rs", base_name.to_lowercase());
//     let mut file = File::create(path)?;

//     let mut tree_types = Vec::new();

//     writeln!(file, "{}", "use crate::error::*;")?;
//     writeln!(file, "{}", "use crate::token::*;\n\n")?;

//     for ttype in types {
//         let (base_class_name, args) = ttype.split_once(":").unwrap();
//         let class_name = format!("{}{}", base_class_name.trim(), base_name); // Binary + Expr
//         let args_split = args.split(",");
//         let mut fields = Vec::new();
//         for arg in args_split {
//             let (t2type, name) = arg.trim().split_once(" ").unwrap();
//             fields.push(format!("{}: {}", name, t2type));
//         }
//         tree_types.push(TreeType {
//             base_class_name: base_class_name.trim().to_string(),
//             class_name,
//             fields,
//         })
//     }

//     write!(file, "pub enum {base_name} {{\n")?;

//     for t in &tree_types {
//         write!(file, "    {}({}),\n", t.base_class_name, t.class_name)?;
//     }
//     write!(file, "}}\n\n")?;

//     for t in &tree_types {
//         write!(file, "pub struct {} {{\n", t.class_name)?;
//         for f in &t.fields {
//             write!(file, "    {},\n", f)?;
//         }
//         write!(file, "}}\n\n")?;
//     }

//     write!(file, "pub trait ExprVisitor<T> {{\n")?;
//     for t in &tree_types {
//         write!(
//             file,
//             "    fn visit_{}_{}(&self, expr: &{}) -> Result<T, KaylanError>;\n",
//             t.base_class_name.to_lowercase(),
//             base_name.to_lowercase(),
//             t.class_name
//         )?;
//     }
//     write!(file, "}}\n\n")?;

//     //println!("{:?}", tree_types);

//     for t in &tree_types {
//         write!(file, "impl {} {{\n", t.class_name)?;
//         write!(
//             file,
//             "    fn accept<T>(&self, visitor:&dyn {}Visitor<T>) -> Result<T, KayLanError> {{\n",
//             base_name
//         )?;
//         write!(
//             file,
//             "        visitor.visit_{}_{}(self)\n",
//             t.base_class_name.to_lowercase(),
//             base_name
//         )?;
//         write!(file, "    }}\n")?;
//         write!(file, "}}\n\n")?;
//     }

//     Ok(())
// }

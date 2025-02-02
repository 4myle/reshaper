
use melody_compiler::compiler;

fn main() {
    let source = r#"1 to 5 of "A";"#;
    let output = compiler(source);
    println!("{}", output.unwrap());
}

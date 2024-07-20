use rhai::Context;

fn main() {

    let code = "fun(x, y) =>\n    a = x + y\n    b = x - y\n    [a,b]\n[m,n]=fun(8,5)";
    let code = r#"
fun(x, y) =>
    a = x + y
    b = x - y
    [a, b]
[m, n] = fun(8, 5)
"#;

    let context = Context::new();
    let result = context.run_ast(code);
    println!("Result: {:?}", result);
    match result {
        Ok(vars) => {
            println!("Variables: {:?}", vars);
            if let Some(m) = vars.get("m") {
                println!("m: {:?}", m);
            }
            if let Some(n) = vars.get("n") {
                println!("n: {:?}", n);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
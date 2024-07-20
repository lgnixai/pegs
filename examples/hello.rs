//! A simple example that evaluates an expression and prints the result.

use rhai::{Engine, EvalAltResult, Scope};
use rhai::ast::atom::Atom;
use rhai::ast::expression::Expression;


fn main() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();
    let mut scope=Scope::new();
    scope.set_variable("z".to_string(),  Expression::Atom(Atom::Double(5.0)));
    let c=engine.run_file("./examples/1.md".into());
     //let scope_clone = scope.clone();

    println!("doc :{:?}",c);


    // if let Some(m) = c.get("a") {
    //     println!("m: {:?}", m);
    // }
    // // engine.run(r#"print("hello, world!") //tests"#)?;
    // // println!("{:?}",engine);
    // // let result = engine.eval::<i64>("40 + 2")?;
    // //
    // // println!("The Answer: {result}"); // prints 42
    // //
    // let mut scope =Scope::new();
    // scope.push("x","hello");
    // // let genv=engine.compile_with_scope(&scope,r#"print(x) //asdf "#)?;
    // // println!("doc :{:?}",genv);
    // // /// engine.run_ast_with_scope(&mut scope, &ast)?;
    // //
    // // let result=engine.run_ast_with_scope(&mut scope,&genv);
    // // println!("doc :{:?}",result);

    Ok(())
}

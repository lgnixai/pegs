use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::ast::expression::Expression;
use crate::ast::function::Function;
use crate::object::object::Object;
use crate::object::ta::TA;
use crate::package::lib::Library;
use crate::package::math::Math;
#[derive(Debug)]
pub struct Scope {
    pub(crate) variables: RefCell<HashMap<String, Expression>>,
    functions: RefCell<HashMap<String, Function>>,
    objects: RefCell<HashMap<String, Object>>,
    imports: RefCell<HashMap<String, HashMap<String, Expression>>>, // Module imports
    libraries:  RefCell<HashMap<String, Box<dyn Library>>>,
}

impl PartialEq for Scope {
    fn eq(&self, other: &Self) -> bool {
        *self.variables.borrow() == *other.variables.borrow() &&
            *self.functions.borrow() == *other.functions.borrow() &&
            *self.objects.borrow() == *other.objects.borrow() &&
            *self.imports.borrow() == *other.imports.borrow()
        // Note: `libraries` comparison is omitted because `Box<dyn Library>` does not implement `PartialEq`
    }
}

impl Clone for Scope {
    fn clone(&self) -> Self {
        Scope {
            variables: RefCell::new(self.variables.borrow().clone()),
            functions: RefCell::new(self.functions.borrow().clone()),
            objects: RefCell::new(self.objects.borrow().clone()),
            imports: RefCell::new(self.imports.borrow().clone()),
            libraries: RefCell::new(HashMap::new()), // Creating an empty HashMap for libraries
        }
    }
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            variables: RefCell::new(HashMap::new()),
            functions: RefCell::new(HashMap::new()),
            objects: RefCell::new(HashMap::new()),
            imports: RefCell::new(HashMap::new()),
            libraries:  RefCell::new(HashMap::new()),
        }
    }

    pub fn set_variable(&self, name: String, value: Expression) {
        self.variables.borrow_mut().insert(name, value);
    }

    pub fn get_variable(&self, name: &str) -> Option<Expression> {
        self.variables.borrow().get(name).cloned()
    }

    pub fn set_function(&self, name: String, function: Function) {
        self.functions.borrow_mut().insert(name, function);
    }

    pub fn get_function(&self, name: &str) -> Option<Function> {
        self.functions.borrow().get(name).cloned()
    }

    pub fn set_object(&self, name: String, object: Object) {
        println!("{:?},{:?}", name, object);
        self.objects.borrow_mut().insert(name, object);
    }

    pub fn get_object(&self, name: &str) -> Option<Object> {
        self.objects.borrow().get(name).cloned()
    }

    pub fn import_module(&mut self, module_name: &str, functions: HashMap<String, Expression>) {
        self.imports.borrow_mut().insert(module_name.to_string(), functions);
    }

    pub fn get_import(&self, module_name: &str, func_name: &str) -> Option<Expression> {
        self.imports.borrow().get(module_name).and_then(|module| module.get(func_name)).cloned()
    }

    pub fn register_library(&mut self, name: &str, library: Box<dyn Library>) {

        //println!("register1:{:?},{:?}",name,library.call_function("abs",&[-3.14]));

        self.libraries.borrow_mut().insert(name.to_string(), library);


        //let dd=self.libraries.borrow().get(name).and_then(|lib| lib.call_function("abs",&[-3.14]));

        //let dd=self.libraries.get("math").call_function("abs",&[-3.14]);
        //println!("register2:{:?},{:?}",name,dd);

    }

    pub fn call_library_function(&self, lib_name: &str, func_name: &str, args: Vec<Expression>) -> Result<Expression, String> {
        println!("call_library_function: lib_name: {:?}, func_name: {:?}, args: {:?}", lib_name, func_name, args);
        println!("libraries: {:?}", self.libraries);

        // Get the library
        if let Some(lib) = self.libraries.borrow().get(lib_name) {

            let cc=lib.call_method(func_name, args);
            println!("----{:?}",cc);
            cc

            // Call the method and handle the Result
            // match lib.call_method(func_name, args) {
            //     Ok(result) => Ok(result), // Return the result
            //     Err(err) => Err(err),     // Return the error
            // }
        } else {
            Err(format!("Library '{}' not found", lib_name)) // Return error if library is not found
        }
    }


    pub fn import_library(&mut self, library_name: &str) {
        match library_name {
            "math" => self.register_library("math", Box::new(Math)),
            _ => eprintln!("Library '{}' is not recognized", library_name),
        }
    }
}
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::thread::scope;

use nom::character::complete::none_of;
use crate::ast::expression::Expression;
use crate::ast::function::Function;
use crate::Scope;

pub struct Engine {
    scopes: Vec<Scope>,
}

impl fmt::Debug for Engine {
    #[cold]
    #[inline(never)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Foo")
            .field("bar", &"333")

            .finish()
    }
}

impl Default for Engine {
    #[inline(always)]
    #[must_use]
    fn default() -> Self {
        Self::new()
    }
}


impl Engine {
    /// An empty raw [`Engine`].
    pub const RAW: Self = Self {
        scopes: Vec::new(),
    };

    /// Create a new [`Engine`].
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        // Create the new scripting Engine
        let mut engine = Self::new_raw();


        engine
    }
    #[inline]
    #[must_use]
    pub const fn new_raw() -> Self {
        Self::RAW
    }


    pub fn register_fn(&mut self, name: String, function: Function) {
        // 注册全局函数
        if let Some(scope) = self.scopes.first_mut() {
            scope.set_function(name, function);
        }
    }



}

#[cfg(test)]
mod tests {
    use crate::ast::vm::VM;

    #[test]
    fn test_vm() {
        let mut vm = VM::new();
        let script = "a = 1; b = a + 2;";
        vm.execute(script);
        assert_eq!(vm.get_variable("b"), Some(3));
    }
}

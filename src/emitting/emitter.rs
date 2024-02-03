use crate::parsing::parser::Statement;
pub struct CEmitter {
    statements: Vec<Statement>,
}

impl CEmitter {
    //todo should program be wrapped?
    pub fn new(program: &[Statement]) -> Self {
        CEmitter {
            statements: program.iter().cloned().collect(),
        }
    }

    pub fn emit(&self) -> String {
        let mut code = String::new();
        code.push_str("#include <stdio.h>\n");
        code.push_str("int main(void){\n");
        for statement in &self.statements {
            // match statement {
            //     Statement::Print { option } => todo!(),
            //     Statement::If {
            //         comparison,
            //         statements,
            //     } => todo!(),
            //     Statement::While {
            //         comparison,
            //         statements,
            //     } => todo!(),
            //     Statement::Let {
            //         identifier,
            //         expression,
            //     } => todo!(),
            //     Statement::Input { identifier } => todo!(),
            //     Statement::Assign {
            //         identifier,
            //         expression,
            //     } => todo!(),
            // };
            // code.push_str("}\n");
        }
        code.push_str("return 0;\n}\n");
        code
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        assert!(true);
    }
}

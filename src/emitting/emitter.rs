use crate::parsing::parser::{
    Comparison, Expression, ExpressionOp, Identifier, Primary, PrintOption, Statement, Term,
    TermOp, Unary,
};
pub struct Indent {
    indent_level: usize,
}

impl Indent {
    pub fn new() -> Self {
        Indent { indent_level: 0 }
    }

    pub fn increase(&mut self) {
        self.indent_level += 1;
    }

    pub fn decrease(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }

    pub fn current_indent(&self) -> String {
        "\t".repeat(self.indent_level)
    }
}
pub struct CEmitter {
    statements: Vec<Statement>,
    code: String,
    indentor: Indent,
}

impl CEmitter {
    //todo should program be wrapped?
    pub fn new(program: &[Statement]) -> Self {
        CEmitter {
            statements: program.to_vec(),
            code: String::new(),
            indentor: Indent::new(),
        }
    }

    pub fn emit(mut self) -> String {
        self.code.push_str("#include <stdio.h>\n");
        self.code.push_str("int main(void){\n");
        self.indentor.increase();
        for statement in self.statements.clone().into_iter() {
            self.code
                .push_str(&Self::emit_statement(&statement, &mut self.indentor));
        }
        self.code.push_str("\treturn 0;\n}\n");
        self.code
    }

    fn emit_statement(statement: &Statement, indent: &mut Indent) -> String {
        let mut emit = String::new();
        emit.push_str(&indent.current_indent());
        match statement {
            Statement::Print { option } => emit.push_str(&Self::emit_print(option)),
            Statement::If {
                comparison,
                statements,
            } => emit.push_str(&Self::emit_if(comparison, statements, indent)),
            Statement::While {
                comparison,
                statements,
            } => emit.push_str(&Self::emit_while(comparison, statements, indent)),
            Statement::Let {
                identifier,
                expression,
            } => emit.push_str(&Self::emit_let(identifier, expression)),
            Statement::Input { identifier } => emit.push_str(&Self::emit_input(identifier, indent)),
            Statement::Assign {
                identifier,
                expression,
            } => emit.push_str(&Self::emit_assign(identifier, expression)),
        };
        emit.push('\n');
        emit
    }

    fn emit_print(print_option: &PrintOption) -> String {
        let mut emit = String::new();
        match print_option {
            PrintOption::PrintLiteral(s) => emit.push_str(format!("printf(\"{}\");", s).as_str()),
            PrintOption::PrintExpression(e) => emit.push_str(
                format!(
                    "printf(\"%.2f\\n\", (float)({}));",
                    &Self::emit_expression(e)
                )
                .as_str(),
            ),
        }
        emit
    }
    fn emit_if(comparison: &Comparison, statements: &[Statement], indent: &mut Indent) -> String {
        let mut emit = String::new();
        emit.push_str(format!("if ({}) {{\n", &Self::emit_comparison(comparison)).as_str());
        indent.increase();
        for statement in statements {
            emit.push_str(Self::emit_statement(statement, indent).as_str())
        }
        indent.decrease();
        emit.push_str(&indent.current_indent());
        emit.push('}');
        emit
    }
    fn emit_while(
        comparison: &Comparison,
        statements: &[Statement],
        indent: &mut Indent,
    ) -> String {
        let mut emit = String::new();
        emit.push_str(format!("while ({}) {{\n", &Self::emit_comparison(comparison)).as_str());
        indent.increase();
        for statement in statements {
            emit.push_str(Self::emit_statement(statement, indent).as_str())
        }
        indent.decrease();
        emit.push_str(&indent.current_indent());

        emit.push('}');
        emit
    }
    fn emit_let(identifier: &Identifier, expression: &Expression) -> String {
        let mut emit = String::new();
        emit.push_str(
            format!(
                "float {} = {};",
                identifier.id,
                &Self::emit_expression(expression)
            )
            .as_str(),
        );
        emit
    }
    fn emit_input(identifier: &Identifier, indent: &mut Indent) -> String {
        // float c;
        // if(0 == scanf("%f", &c)) {
        //     c = 0;
        //     scanf("%*s");
        //     }
        let mut emit = String::new();
        emit.push_str(
            format!(
                "float {};\n{}if(0==scanf(\"%f\", &{})) {{\n{}\t{} = 0;\n{}\tscanf(\"%*s\");\n{}}}",
                identifier.id,
                &indent.current_indent(),
                identifier.id,
                &indent.current_indent(),
                identifier.id,
                &indent.current_indent(),
                &indent.current_indent()
            )
            .as_str(),
        );
        emit
    }
    fn emit_assign(identifier: &Identifier, expression: &Expression) -> String {
        let mut emit = String::new();
        emit.push_str(
            format!(
                "{} = {};",
                identifier.id,
                &Self::emit_expression(expression)
            )
            .as_str(),
        );
        emit
    }
    fn emit_comparison(comparison: &Comparison) -> String {
        let mut emit = String::new();
        let (lhs, rhs, sign) = match comparison {
            Comparison::GreaterThan { lhs, rhs } => (lhs, rhs, ">"),
            Comparison::GreaterThanEquals { lhs, rhs } => (lhs, rhs, ">="),
            Comparison::LessThan { lhs, rhs } => (lhs, rhs, "<"),
            Comparison::LessThanEquals { lhs, rhs } => (lhs, rhs, "<="),
            Comparison::EqualsEquals { lhs, rhs } => (lhs, rhs, "=="),
            Comparison::NotEquals { lhs, rhs } => (lhs, rhs, "!="),
        };
        emit.push_str(
            format!(
                "{} {} {}",
                &Self::emit_expression(lhs),
                sign,
                &Self::emit_expression(rhs)
            )
            .as_str(),
        );
        emit
    }
    fn emit_expression(expression: &Expression) -> String {
        let mut emit = String::new();
        let rhs = match expression.rhs.as_ref() {
            Some(ExpressionOp::Plus(e)) => format!(" + {}", &Self::emit_expression(e)),
            Some(ExpressionOp::Minus(e)) => format!(" - {}", &Self::emit_expression(e)),
            None => "".to_string(),
        };
        emit.push_str(format!("{}{}", &Self::emit_term(&expression.lhs), rhs).as_str());
        emit
    }

    fn emit_term(term: &Term) -> String {
        let mut emit = String::new();
        let rhs = match term.rhs.as_ref() {
            Some(TermOp::Multiply(t)) => format!(" * {}", &Self::emit_term(t)),
            Some(TermOp::Divide(t)) => format!(" / {}", &Self::emit_term(t)),
            None => "".to_string(),
        };
        emit.push_str(format!("{}{}", &Self::emit_unary(&term.lhs), rhs).as_str());
        emit
    }
    fn emit_unary(unary: &Unary) -> String {
        let mut emit = String::new();
        let str = match unary {
            Unary::Positive(p) => format!("+{}", &Self::emit_primary(p)),
            Unary::Negative(p) => format!("-{}", &Self::emit_primary(p)),
            Unary::UnSigned(p) => Self::emit_primary(p),
        };
        emit.push_str(&str);
        emit
    }
    fn emit_primary(primary: &Primary) -> String {
        let mut emit = String::new();
        let str = match primary {
            Primary::LiteralNumber(n) => n.to_string(),
            Primary::IdentifierExpression(id) => id.id.clone(),
        };
        emit.push_str(&str);
        emit
    }
}

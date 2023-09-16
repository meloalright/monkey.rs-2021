use crate::ast;
pub mod env;
pub mod object;

#[derive(PartialEq, Clone, Debug)]
pub struct Evaluator {
    pub env: env::Env
}

impl Evaluator {
    pub fn eval(&mut self, program: &ast::Program) -> Option<object::Object> {
        let mut result = None;
        
        for stmt in program {
            match self.eval_stmt(stmt) {
                Some(object::Object::ReturnValue(value)) => return Some(*value),
                obj => { println!("env {:?} obj {:?}", self.env, obj); result = obj }
            }
        }
        result
    }

    fn eval_stmt(&mut self, stmt: &ast::Stmt) -> Option<object::Object> {
        match stmt {
            ast::Stmt::Let(ident, expr) => {
                let value = match self.eval_expr(expr) {
                    Some(value) => value,
                    None => return None,
                };
                let ast::Ident(name) = ident;
                self.env.set(name.clone(), value);
                None
            },
            ast::Stmt::Return(expr) => {
                let value = match self.eval_expr(expr) {
                    Some(value) => value,
                    None => return None,
                };
                Some(object::Object::ReturnValue(Box::new(value)))
            }
            ast::Stmt::Expr(expr) => self.eval_expr(expr),
            ast::Stmt::ReAssign(ident, expr) => {
                let value = match self.eval_expr(expr) {
                    Some(value) => value,
                    None => return None,
                };
                let ast::Ident(name) = ident;
                self.env.set(name.clone(), value);
                None
            }
            _ => todo!()
        }
    }

    fn eval_expr(&mut self, expr: &ast::Expr) -> Option<object::Object> {
        match expr {
            ast::Expr::Ident(ident) => Some(self.eval_ident(ident)),
            ast::Expr::Literal(literal) => Some(self.eval_literal(literal)),
            ast::Expr::While { cond, consequence } => self.eval_while_expr(&*cond, consequence),
            _ => { None }
        }
    }

    fn eval_ident(&mut self, ident: &ast::Ident) -> object::Object {
        let ast::Ident(name) = ident;

        match self.env.get(name.clone()) {
            Some(value) => value,
            None => panic!(),
        }
    }


    fn eval_literal(&mut self, literal: &ast::Literal) -> object::Object {
        match literal {
            ast::Literal::Int(value) => object::Object::Int(*value),
            _ => panic!()
        }
    }

    fn eval_while_expr(&mut self, cond: &ast::Expr, consequence: &ast::BlockStmt) -> Option<object::Object> {
        let mut result: Option<object::Object> = None;
        loop {
            let cond_result = match self.eval_expr(cond) {
                Some(cond) => cond,
                None => break,
            };
            if !Self::is_truthy(cond_result.clone()) {
                break;
            }
            self.eval_block_stmt_with_continue_and_break_statement(consequence);
        }
        result
    }



    fn eval_block_stmt_with_continue_and_break_statement(&mut self, stmts: &ast::BlockStmt) -> Option<object::Object> {
        let mut result = None;

        for stmt in stmts {
            // if *stmt == ast::Stmt::Blank {
            //     continue;
            // }

            match self.eval_stmt(stmt) {
                // Some(object::Object::ReturnValue(value)) => return Some(object::Object::ReturnValue(value)),
                // Some(Object::BreakStatement) => return Some(Object::BreakStatement),
                // Some(Object::ContinueStatement) => return Some(Object::ContinueStatement),
                // Some(Object::Error(msg)) => return Some(Object::Error(msg)),
                obj => result = obj,
                _ => todo!()
            }
        }

        result
    }



    fn is_truthy(obj: object::Object) -> bool {
        match obj {
            object::Object::Int(0) => false,
            // todo
            _ => true,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use super::Evaluator;
    use super::env;
    use super::object;

    #[test]
    fn test_let_evaluator() {
        let mut lexer = Lexer::new(r"let five = 5;");
        let mut parser = Parser::new(lexer);
        let program = parser.parse();
        let mut evaluator = Evaluator { env: env::Env::new() };
        evaluator.eval(&program);
    }

    #[test]
    fn test_return_evaluator() {
        let mut lexer = Lexer::new(r"return 3");
        let mut parser = Parser::new(lexer);
        let program = parser.parse();
        let mut evaluator = Evaluator { env: env::Env::new() };
        let rt = evaluator.eval(&program);
        match rt {
            Some(object::Object::Int(v)) => {
                assert_eq!(v, 3i64);
            },
            _ => todo!()
        }
    }

    #[test]
    fn test_expr_evaluator() {
        let mut lexer = Lexer::new(r"6");
        let mut parser = Parser::new(lexer);
        let program = parser.parse();
        let mut evaluator = Evaluator { env: env::Env::new() };
        let rt = evaluator.eval(&program);
        match rt {
            Some(object::Object::Int(v)) => {
                assert_eq!(v, 6i64);
            },
            _ => todo!()
        }
    }

    #[test]
    fn test_reassign_evaluator() {
        let mut lexer = Lexer::new(r"let five = 5; five = 6");
        let mut parser = Parser::new(lexer);
        let program = parser.parse();
        let mut evaluator = Evaluator { env: env::Env::new() };
        evaluator.eval(&program);
    }

    #[test]
    fn test_while_evaluator() {
        let mut lexer = Lexer::new(r"let a = 5; while (a) { a = 0; }");
        let mut parser = Parser::new(lexer);
        let program = parser.parse();
        let mut evaluator = Evaluator { env: env::Env::new() };
        evaluator.eval(&program);
    }
}
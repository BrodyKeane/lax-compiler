use crate::token::Token;

pub trait Data {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output;
}

pub trait Visitor {
    type Output;

    fn visit_binary_expr(&mut self, expr: &Binary) -> Self::Output;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Self::Output;
    fn visit_literal_expr(&mut self, expr: &Literal) -> Self::Output;
    fn visit_unary_expr(&mut self, expr: &Unary) -> Self::Output;
//    fn visit_assign_expr(&mut self, expr: &Assign) -> Self::Output;
//    fn visit_call_expr(&mut self, expr: &Call) -> Self::Output;
//    fn visit_get_expr(&mut self, expr: &Get) -> Self::Output;
//    fn visit_logical_expr(&mut self, expr: &Logical) -> Self::Output;
//    fn visit_set_expr(&mut self, expr: &Set) -> Self::Output;
//    fn visit_super_expr(&mut self, expr: &Super) -> Self::Output;
//    fn visit_this_expr(&mut self, expr: &This) -> Self::Output;
//    fn visit_variable_expr(&mut self, expr: &Variable) -> Self::Output;
}


pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

impl Data for Expr {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        match self {
            Expr::Binary(expr) => visitor.visit_binary_expr(expr),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Literal(expr) => visitor.visit_literal_expr(expr),
            Expr::Unary(expr) => visitor.visit_unary_expr(expr),
        }
    }
}

impl Expr {
    pub fn binary(left: Expr, operator: Token, right: Expr) -> Self {
        Expr::Binary(Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
    
    pub fn grouping(expr: Expr) -> Self {
        Expr::Grouping(Grouping {
            expr: Box::new(expr),
        })
    }

    pub fn literal(value: Option<String>) -> Self {
        Expr::Literal(Literal { value })
    }

    pub fn unary(operator: Token, right: Expr) -> Self {
        Expr::Unary(Unary {
            operator,
            right: Box::new(right),
        })
    }
}

pub struct Binary {
  pub left: Box<Expr>,
  pub operator: Token,
  pub right: Box<Expr>,
}

pub struct Grouping {
    pub expr: Box<Expr>, 
}

pub struct Literal {
    pub value: Option<String>,
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

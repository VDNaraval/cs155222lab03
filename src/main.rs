use ArithCmpOp::*;
use ArithExpr::*;
use BinArithOp::*;
use BinLogicOp::*;
use BoolExpr::*;
use Expr::*;
use Value::*;

pub enum Expr {
    ArithExpr(ArithExpr),
    BoolExpr(BoolExpr),
}

pub enum ArithExpr {
    BinArithExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: BinArithOp,
    },
    IntLit(i64),
}

pub enum BoolExpr {
    ArithCmpExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: ArithCmpOp,
    },
    BinBoolExpr {
        left: Box<BoolExpr>,
        right: Box<BoolExpr>,
        op: BinLogicOp,
    },
    NotExpr(Box<BoolExpr>),
    BoolLit(bool),
}

pub enum BinArithOp {
    AddOp,
    SubOp,
    MulOp,
    IntDivOp,
}

pub enum ArithCmpOp {
    LtOp,
    LteOp,
    GtOp,
    GteOp,
    ArithEqOp,
    ArithNeqOp,
}

pub enum BinLogicOp {
    AndOp,
    OrOp,
    BoolEqOp,
    BoolNeqOp,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    BoolValue(bool),
    IntValue(i64),
}

pub fn eval(expr: Expr) -> Value {
    match expr {
        ArithExpr(expr)    => IntValue(eval_arith_expr(expr)),
        BoolExpr(expr)      => BoolValue(eval_bool_expr(expr)),
    }
}

pub fn eval_arith_expr(arith_expr: ArithExpr) -> i64 {
    match arith_expr {
        BinArithExpr {left, right, op}  => {
            match op {
                AddOp       => eval_arith_expr(*left) + eval_arith_expr(*right),
                SubOp       => eval_arith_expr(*left) - eval_arith_expr(*right),
                MulOp       => eval_arith_expr(*left) * eval_arith_expr(*right),
                IntDivOp    => eval_arith_expr(*left) / eval_arith_expr(*right),
            }
        }
        IntLit(num)     => num,
    }
}

pub fn eval_bool_expr(bool_expr: BoolExpr) -> bool {
    match bool_expr {
        ArithCmpExpr {left, right, op}  => {
            match op {
                LtOp        => eval_arith_expr(*left) < eval_arith_expr(*right),
                LteOp       => eval_arith_expr(*left) <= eval_arith_expr(*right),
                GtOp        => eval_arith_expr(*left) > eval_arith_expr(*right),
                GteOp       => eval_arith_expr(*left) >= eval_arith_expr(*right),
                ArithEqOp   => eval_arith_expr(*left) == eval_arith_expr(*right),
                ArithNeqOp  => eval_arith_expr(*left) != eval_arith_expr(*right),
            }
        }
        BinBoolExpr {left, right, op}   => {
            match op {
                AndOp       => eval_bool_expr(*left) && eval_bool_expr(*right),
                OrOp        => eval_bool_expr(*left) || eval_bool_expr(*right),
                BoolEqOp    => eval_bool_expr(*left) == eval_bool_expr(*right),
                BoolNeqOp   => eval_bool_expr(*left) != eval_bool_expr(*right),
            }
        }
        NotExpr(expr)   => !eval_bool_expr(*expr),
        BoolLit(boolean)         => boolean,
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int() {
        let expr = ArithExpr(IntLit(32));
        assert_eq!(eval(expr), IntValue(32));
    }

    #[test]
    fn test_bin_arith_add() {
        let expr = ArithExpr(BinArithExpr { left: Box::new(IntLit(24)), right: Box::new(IntLit(8)), op: AddOp });
        assert_eq!(eval(expr), IntValue(32));
    }

    #[test]
    fn test_bin_arith_sub() {
        let expr = ArithExpr(BinArithExpr { left: Box::new(IntLit(40)), right: Box::new(IntLit(8)), op: SubOp });
        assert_eq!(eval(expr), IntValue(32));
    }

    #[test]
    fn test_bin_arith_mul() {
        let expr = ArithExpr(BinArithExpr { left: Box::new(IntLit(8)), right: Box::new(IntLit(4)), op: MulOp });
        assert_eq!(eval(expr), IntValue(32));
    }

    #[test]
    fn test_bin_arith_int_div() {
        let expr = ArithExpr(BinArithExpr { left: Box::new(IntLit(64)), right: Box::new(IntLit(2)), op: IntDivOp });
        assert_eq!(eval(expr), IntValue(32));
    }

    #[test]
    fn test_bool() {
        let expr = BoolExpr(BoolLit(true));
        assert_eq!(eval(expr), BoolValue(true));
    }

    #[test]
    fn test_not() {
        let expr = BoolExpr(NotExpr(Box::new(BoolLit(false))));
        assert_eq!(eval(expr), BoolValue(true));
    }

    #[test]
    fn test_arith_cmp_lt() {
        let expr = BoolExpr(ArithCmpExpr { left: Box::new(IntLit(32)), right: Box::new(IntLit(64)), op: LtOp });
        assert_eq!(eval(expr), BoolValue(true));
    }

    #[test]
    fn test_arith_cmp_lte() {
        let expr = BoolExpr(ArithCmpExpr { left: Box::new(IntLit(32)), right: Box::new(IntLit(32)), op: LteOp });
        assert_eq!(eval(expr), BoolValue(true));
    }

    #[test]
    fn test_arith_cmp_gt() {
        let expr = BoolExpr(ArithCmpExpr { left: Box::new(IntLit(64)), right: Box::new(IntLit(32)), op: GtOp });
        assert_eq!(eval(expr), BoolValue(true));
    }

    #[test]
    fn test_arith_cmp_gte() {
        let expr = BoolExpr(ArithCmpExpr { left: Box::new(IntLit(64)), right: Box::new(IntLit(64)), op: GteOp });
        assert_eq!(eval(expr), BoolValue(true));
    }

    #[test]
    fn test_arith_cmp_arith_eq() {
        let expr = BoolExpr(ArithCmpExpr { left: Box::new(IntLit(32)), right: Box::new(IntLit(32)), op: ArithEqOp });
        assert_eq!(eval(expr), BoolValue(true));
    }

    #[test]
    fn test_arith_cmp_arith_neq() {
        let expr = BoolExpr(ArithCmpExpr { left: Box::new(IntLit(32)), right: Box::new(IntLit(64)), op: ArithNeqOp });
        assert_eq!(eval(expr), BoolValue(true));
    }

    #[test]
    fn test_bin_bool_and() {
        let expr = BoolExpr(BinBoolExpr { left: Box::new(BoolLit(true)), right: Box::new(BoolLit(true)), op: AndOp });
        assert_eq!(eval(expr), BoolValue(true));
    }

    #[test]
    fn test_bin_bool_or() {
        let expr = BoolExpr(BinBoolExpr { left: Box::new(BoolLit(true)), right: Box::new(BoolLit(false)), op: OrOp });
        assert_eq!(eval(expr), BoolValue(true));
    }

    #[test]
    fn test_bin_bool_eq() {
        let expr = BoolExpr(BinBoolExpr { left: Box::new(BoolLit(true)), right: Box::new(BoolLit(true)), op: BoolEqOp });
        assert_eq!(eval(expr), BoolValue(true));
    }

    #[test]
    fn test_bin_bool_neq() {
        let expr = BoolExpr(BinBoolExpr { left: Box::new(BoolLit(true)), right: Box::new(BoolLit(false)), op: BoolNeqOp });
        assert_eq!(eval(expr), BoolValue(true));
    }

    #[test]
    fn test_others() {
        main();
        println!("{:?}", BoolValue(true));
    }
}
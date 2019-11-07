use egg::{
    define_term,
    egraph::{EClass},
    expr::{Expr, Language, Name, RecExpr},
};

use num_traits::{Zero};
use num_rational::{Ratio, BigRational};

pub type MathEGraph<M = Meta> = egg::egraph::EGraph<Math, M>;


define_term! {
    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    pub enum FPConstant {
    True = "TRUE",
    False = "FALSE",
    E = "E",
    Log2E = "LOG2E",
    Log10E = "LOG10E",
    Ln2 = "LN2",
    Ln10 = "LN10",
    Pi = "PI",
    Pi2 = "PI_2",
    Pi4 = "PI_4",
    Pi1Alt = "1_PI",
    Pi2Alt = "2_PI",
    Sqrtpi2 = "2_SQRTPI",
    Sqrt2 = "SQRT2",
    Sqrt1_2 = "SQRT1_2",
    Infinity = "INFINITY",
    Nan = "NAN",
    }
}

type Constant = BigRational;
// operators from FPCore
define_term! {
    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    pub enum Math {
        Constant(Constant),

	// complex operators not from FPCore
	Re = "re",
	Im = "im",
	Complex = "complex",
	Conj = "conj",
	Addc = "+.c",
	Subc = "-.c",
	Negc = "neg.c",
	Divc = "/.c",
	Mulc = "*.c",


	// FPCore operations
	Erf = "erf",
	Erfc = "erfc",
	Tgamma = "tgamma",
	Lgamma = "lgamma",
	Ceil = "ceil",
	Floor = "floor",
	Fmod = "fmod",
	Remainder = "remainder",
	Fmax = "fmax",
	Fmin = "fmin",
	Fdim = "fdim",
	Copysign = "copysign",
	Trunc = "trunc",
	Round = "round",
	NearbyInt = "nearbyint",



        Add = "+",
        Sub = "-",
        Mul = "*",
        Div = "/",
        Pow = "pow",
        Exp = "exp",
	Exp2 = "exp2",
        Log = "log",
        Sqrt = "sqrt",
        Cbrt = "cbrt",
        Fabs = "fabs",
        Sin = "sin",
        Cos = "cos",
        Tan = "tan",
        Asin = "asin",
        Acos = "acos",
        Atan = "atan",
        Atan2 = "atan2",
        Sinh = "sinh",
        Cosh = "cosh",
        Tanh = "tanh",
        Asinh = "asinh",
        Acosh = "acosh",
        Atanh = "atanh",

        Fma = "fma",
        Log1p = "log1p",
	Log10 = "log10",
	Log2 = "log2",
        Expm1 = "expm1",
        Hypot = "hypot",

        PositAdd = "+.p16",
        PositSub = "-.p16",
        PositMul = "*.p16",
        PositDiv = "/.p16",
        RealToPosit = "real->posit",
	FPConstant(FPConstant),
        Variable(Name),
    }
}

impl Language for Math {
    fn cost(&self, children: &[u64]) -> u64 {
        let cost = match self {
            Math::Constant(_) | Math::Variable(_) | Math::FPConstant(_) => 0,
            _ => 1,
        };

        cost + children.iter().sum::<u64>()
    }
}

#[derive(Debug, Clone)]
pub struct Meta {
    pub cost: u64,
    pub best: RecExpr<Math>,
}

fn eval(op: Math, args: &[Constant]) -> Option<Constant> {
    let a = |i| args.get(i).cloned();
    match op {
        Math::Add => Some(a(0)? + a(1)?),
        Math::Sub => Some(a(0)? - a(1)?),
        Math::Mul => Some(a(0)? * a(1)?),
        Math::Div => {
            if a(1)?.is_zero() {
                None
            } else {
                Some(a(0)? / a(1)?)
            }
        }
        Math::Pow => None, // a(0)?.powf(a(1)?),
        Math::Exp => None, // a(0)?.exp(),
        Math::Log => None, // a(0)?.ln(),
        Math::Sqrt => {
            None
            // unimplemented!()
            // if let Some(sqrt) = args[0].sqrt() {
            //     #[allow(clippy::float_cmp)]
            //     let is_int = sqrt == sqrt.trunc();
            //     if is_int {
            //         sqrt.into()
            //     } else {
            //         None
            //     }
            // } else {
            //     None
            // }
        }
        // Math::Cbrt => {
        //     if let Some(cbrt) = args[0].to_f64().map(f64::cbrt) {
        //         #[allow(clippy::float_cmp)]
        //         let is_int = cbrt == cbrt.trunc();
        //         if is_int {
        //             cbrt.into()
        //         } else {
        //             None
        //         }
        //     } else {
        //         None
        //     }
        // }
        Math::Fabs => {
            if a(0)? < Ratio::from_integer(Zero::zero()) {
                Some(-a(0)?)
            } else {
                Some(a(0)?)
            }
        }
        Math::RealToPosit => Some(a(0)?),
        _ => None,
    }
}

impl egg::egraph::Metadata<Math> for Meta {
    type Error = std::convert::Infallible;
    fn merge(&self, other: &Self) -> Self {
        if self.cost <= other.cost {
            self.clone()
        } else {
            other.clone()
        }
    }

    fn make(expr: Expr<Math, &Self>) -> Self {
        let expr = {
            let const_args: Option<Vec<Constant>> = expr
                .children
                .iter()
                .map(|meta| match meta.best.as_ref().op {
                    Math::Constant(ref c) => Some(c.clone()),
                    _ => None,
                })
                .collect();

            const_args
                .and_then(|a| eval(expr.op.clone(), &a))
                .map(|c| Expr::unit(Math::Constant(c)))
                .unwrap_or(expr)
        };

        let best: RecExpr<_> = expr.map_children(|c| c.best.clone()).into();
        Self {
            best,
            cost: expr.map_children(|c| c.cost).cost(),
        }
    }

    fn modify(eclass: &mut EClass<Math, Self>) {
	
        // NOTE pruning vs not pruning is decided right here
        let best = eclass.metadata.best.as_ref();
        if best.children.is_empty() {
            eclass.nodes.push(Expr::unit(best.op.clone()));
        }
    }
}

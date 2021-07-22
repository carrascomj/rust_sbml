use super::UnitSIdRef;
use serde::{Deserialize, Serialize};

/// Math attribute which contains MathNodes of the very partially implemented
/// [MathML version 3.0 spec](https://www.w3.org/TR/2014/REC-MathML3-20140410).
#[derive(Deserialize, Debug, Serialize, Eq, PartialEq, Clone)]
pub struct Math {
    #[serde(rename = "$value")]
    content: MathNode,
}

/// Content identifier <ci>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Apply {
    #[serde(rename = "$value")]
    content: Vec<MathNode>,
}

/// Content identifier <ci>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Ci {
    #[serde(rename = "$value")]
    content: String,
    #[serde(rename = "type")]
    ci_type: Option<String>,
}

#[derive(Debug, Serialize, Eq, PartialEq, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Number {
    Real,
    Integer,
    Rational,
    ComplexCartesian,
    ComplexPolar,
    Constant,
    ENotation,
}

impl Default for Number {
    fn default() -> Self {
        Number::Real
    }
}

/// Numbers <cn>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Cn {
    #[serde(rename = "$value")]
    content: String,
    #[serde(rename = "sbml:units")]
    unit: Option<UnitSIdRef>,
    #[serde(rename = "type", default)]
    ci_type: Number,
}

/// Main node of MathML
///
/// Very partial implementation
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MathNode {
    Apply(Box<Apply>),
    Text(String),
    Ci(Ci),
    Csymbol {
        cd: Option<String>,
        encoding: Option<String>,
        #[serde(rename = "$value")]
        children: Vec<MathNode>,
    },
    Cn(Cn),
    Comment(String),
    PI(String, Option<String>),
    // rest of operations
    Factorial,
    Minus,
    Abs,
    Conjugate,
    Arg,
    Real,
    Imaginary,
    Floor,
    Ceiling,
    Not,
    Inverse,
    Ident,
    Domain,
    Codomain,
    Image,
    Sin,
    Cos,
    Tan,
    Sec,
    Csc,
    Cot,
    Sinh,
    Cosh,
    Tanh,
    Sech,
    Csch,
    Coth,
    Arcsin,
    Arccos,
    Arctan,
    Arccosh,
    Arccot,
    Arccoth,
    Arccsc,
    Arccsch,
    Arcsec,
    Arcsech,
    Arcsinh,
    Arctanh,
    Exp,
    Ln,
    Log,
    Determinant,
    Transpose,
    Divergence,
    Grad,
    Curl,
    Laplacian,
    Card,
    Quotient,
    Divide,
    Power,
    Rem,
    Implies,
    Equivalent,
    Approx,
    Setdiff,
    Vectorproduct,
    Scalarproduct,
    Outerproduct,
    Plus,
    Times,
    Max,
    Min,
    Gcd,
    Lcm,
    Mean,
    Sdev,
    Variance,
    Median,
    Mode,
    And,
    Or,
    Xor,
    Selector,
    Union,
    Intersect,
    Cartesianproduct,
    Compose,
    #[serde(rename = "fn")]
    Fun,
    Int,
    Sum,
    Product,
    Diff,
    Partialdiff,
    Forall,
    Exists,
    Eq,
    Neq,
    Gt,
    Lt,
    Geq,
    Leq,
    Root,
}

use super::UnitSIdRef;
use serde::{Deserialize, Serialize};

/// Math attribute which contains MathNodes of the very partially implemented
/// [MathML version 3.0 spec](https://www.w3.org/TR/2014/REC-MathML3-20140410).
#[derive(Deserialize, Debug, Serialize, Eq, PartialEq, Clone)]
pub struct Math {
    #[serde(rename = "$value")]
    pub content: MathNode,
}

/// Content identifier <ci>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Apply {
    #[serde(rename = "$value")]
    pub content: Vec<MathNode>,
}

/// Content identifier <ci>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Ci {
    #[serde(rename = "$value")]
    content: String,
    #[serde(rename = "type")]
    ci_type: Option<String>,
}

/// Content identifier <ci>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename = "lowercase")]
pub struct Bvar {
    #[serde(rename = "$value")]
    ci: Ci,
    ci_type: Option<String>,
}

/// Number type (default to Real)
#[derive(Debug, Serialize, Eq, PartialEq, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum NumberType {
    Real,
    Integer,
    Rational,
    ComplexCartesian,
    ComplexPolar,
    Constant,
    ENotation,
}

impl Default for NumberType {
    fn default() -> Self {
        Self::Real
    }
}

/// Base of a number (default to 10)
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Base(pub u32);

impl Default for Base {
    fn default() -> Self {
        Base(10)
    }
}

/// Numbers <cn>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cn {
    #[serde(rename = "$value")]
    pub content: String,
    #[serde(rename = "sbml:units")]
    pub unit: Option<UnitSIdRef>,
    #[serde(rename = "type", default)]
    pub cn_type: NumberType,
    #[serde(default)]
    pub base: Base,
    pub definition_url: Option<String>,
    pub encoding: Option<String>,
}

/// Main node of MathML
///
/// Very partial implementation
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MathNode {
    Apply(Box<Apply>),
    Text(String),
    Ci {
        #[serde(rename = "$value")]
        content: String,
        #[serde(rename = "type")]
        ci_type: Option<String>,
    },
    Csymbol {
        cd: Option<String>,
        encoding: Option<String>,
        #[serde(rename = "$value")]
        children: Vec<MathNode>,
    },
    Cn(Cn),
    Comment(String),
    PI(String, Option<String>),
    Lambda {
        #[serde(rename = "$value")]
        children: Vec<MathNode>,
    },
    Bvar,
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

impl MathNode {
    pub fn apply(x: Vec<MathNode>) -> Self {
        MathNode::Apply(Box::new(Apply { content: x }))
    }
}

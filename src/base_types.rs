use mathml::MathNode;
use roxmltree;
use roxmltree::Attribute;
use roxmltree::Node;
use serde_derive::Deserialize;
use serde_plain;
use std::collections::HashMap;
#[derive(Debug, Deserialize, PartialEq)]
pub enum UnitSidRef {
    SIUnit(UnitSId),
    CustomUnit(String),
}
impl<T: AsRef<str> + ToString> From<&T> for UnitSidRef {
    fn from(r: &T) -> Self {
        match serde_plain::from_str(r.as_ref()) {
            Ok(r) => Self::SIUnit(r),
            Err(_) => Self::CustomUnit(r.to_string()),
        }
    }
}
#[derive(Debug, Hash, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UnitSId {
    Ampere,
    Avogadro,
    Coulomb,
    Gray,
    Joule,
    Litre,
    Mole,
    Radian,
    Steradian,
    Weber,
    Dimensionless,
    Henry,
    Katal,
    Lumen,
    Newton,
    Tesla,
    Becquerel,
    Farad,
    Hertz,
    Kelvin,
    Lux,
    Ohm,
    Siemens,
    Volt,
    Candela,
    Gram,
    Item,
    Kilogram,
    Metre,
    Pascal,
    Sievert,
    Watt,
    Second,
}
#[derive(Debug, Default, PartialEq)]

pub struct ModelUnits {
    pub substance_units: Option<UnitSidRef>,
    pub time_units: Option<UnitSidRef>,
    pub extent_units: Option<UnitSidRef>,
    pub volume_units: Option<UnitSidRef>,
    pub area_units: Option<UnitSidRef>,
    pub length_units: Option<UnitSidRef>,
    pub conversion_factor: Option<UnitSidRef>,
}

impl From<&[Attribute<'_>]> for ModelUnits {
    fn from(value: &[Attribute<'_>]) -> Self {
        let hmap: HashMap<String, String> = value
            .iter()
            .map(|a| (a.name().to_owned(), a.value().to_owned()))
            .collect();
        ModelUnits {
            substance_units: hmap.get("substanceUnits").map(|p| p.into()),
            time_units: hmap.get("timeUnits").map(|p| p.into()),
            extent_units: hmap.get("extentUnits").map(|p| p.into()),
            volume_units: hmap.get("volumeUnits").map(|p| p.into()),
            area_units: hmap.get("areaUnits").map(|p| p.into()),
            length_units: hmap.get("lengthUnits").map(|p| p.into()),
            conversion_factor: hmap.get("conversionFactor").map(|p| p.into()),
        }
    }
}
#[derive(Debug, PartialEq)]
pub struct Unit {
    exponent: f64,
    scale: i64,
    multiplier: f64,
}
impl From<Node<'_, '_>> for Unit {
    fn from(value: Node<'_, '_>) -> Self {
        Unit {
            exponent: value.attribute("exponent").unwrap().parse().unwrap(),
            scale: value.attribute("scale").unwrap().parse().unwrap(),
            multiplier: value.attribute("multiplier").unwrap().parse().unwrap(),
        }
    }
}
#[derive(Debug, PartialEq)]
pub struct Compartment {
    units: Option<UnitSidRef>,
    spatial_dimensions: Option<f64>,
    size: Option<f64>,
    constant: bool,
}
impl From<Node<'_, '_>> for Compartment {
    fn from(value: Node<'_, '_>) -> Self {
        Compartment {
            spatial_dimensions: value
                .attribute("spatialDimensions")
                .map(|p| p.parse().unwrap()),
            size: value.attribute("size").map(|p| p.parse().unwrap()),
            constant: value.attribute("constant").unwrap().parse().unwrap(),
            units: value.attribute("units").map(|p| UnitSidRef::from(&p)),
        }
    }
}
#[derive(Debug, PartialEq)]
pub struct Specie {
    compartment: String,
    initial_concentration: Option<f64>,
    initial_amount: Option<f64>,
    substance_units: Option<UnitSidRef>,
    has_only_substance_units: bool,
    boundary_condition: bool,
    constant: bool,
    conversion_factor: Option<String>,
}
impl<'a> From<Node<'a, 'a>> for Specie {
    fn from(value: Node<'a, 'a>) -> Self {
        Specie {
            compartment: value.attribute("compartment").unwrap().to_owned(),
            initial_concentration: value
                .attribute("initialConcentration")
                .map(|p| p.parse().unwrap()),
            initial_amount: value.attribute("initialAmount").map(|p| p.parse().unwrap()),
            substance_units: value
                .attribute("substanceUnits")
                .map(|p| UnitSidRef::from(&p)),
            has_only_substance_units: value
                .attribute("hasOnlySubstanceUnits")
                .unwrap()
                .parse()
                .unwrap(),
            boundary_condition: value
                .attribute("boundaryCondition")
                .unwrap()
                .parse()
                .unwrap(),
            constant: value.attribute("constant").unwrap().parse().unwrap(),
            conversion_factor: value.attribute("conversionFactor").map(|r| r.to_owned()),
        }
    }
}
#[derive(Debug, PartialEq)]
pub struct Parameter {
    value: Option<f64>,
    units: Option<UnitSidRef>,
    constant: bool,
}
impl<'a> From<Node<'a, 'a>> for Parameter {
    fn from(value: Node<'a, 'a>) -> Self {
        Parameter {
            value: value.attribute("value").map(|p| p.parse().unwrap()),
            units: value.attribute("units").map(|p| UnitSidRef::from(&p)),
            constant: value.attribute("constant").unwrap().parse().unwrap(),
        }
    }
}
#[derive(Debug, PartialEq)]
pub struct InitialAssignment {
    pub symbol: String,
}
type HL<T> = HashMap<String, T>;
#[derive(Debug, Default, PartialEq)]
pub struct Model {
    pub model_units: ModelUnits,
    pub initial_assignments: HL<InitialAssignment>,
    pub parameters: HL<Parameter>,
    pub species: HL<Specie>,
    pub compartments: HL<Compartment>,
    pub unit_definitions: HL<HashMap<UnitSId, Unit>>,
    pub constraints: Vec<Constraint>,
}

#[derive(Debug, PartialEq)]
pub struct Function {
    math: MathNode,
}
#[derive(Debug, PartialEq)]
pub enum Rule<'a> {
    AlgebraicRule { math: MathNode },
    AssignmentRule { math: MathNode, variable: &'a str },
    RateRule { math: MathNode, variable: &'a str },
}
#[derive(Debug, PartialEq)]
pub struct Constraint {
    pub(crate) math: Option<MathNode>,
    pub(crate) message: String,
}

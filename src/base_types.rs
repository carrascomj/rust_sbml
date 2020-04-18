use roxmltree;
use roxmltree::Attribute;
use roxmltree::Node;
use serde_derive::Deserialize;
use serde_plain;
use std::collections::HashMap;
#[derive(Debug, Deserialize)]
pub enum UnitSidRef {
    SIUnit(UnitSId),
    CustomUnit(String),
}
impl From<&str> for UnitSidRef {
    fn from(r: &str) -> Self {
        match serde_plain::from_str(r) {
            Ok(r) => Self::SIUnit(r),
            Err(_) => Self::CustomUnit(r.to_owned()),
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
#[derive(Debug)]

pub struct ModelUnits {
    substance_units: Option<UnitSidRef>,
    time_units: Option<UnitSidRef>,
    extent_units: Option<UnitSidRef>,
    volume_units: Option<UnitSidRef>,
    area_units: Option<UnitSidRef>,
    length_units: Option<UnitSidRef>,
    conversion_factor: Option<UnitSidRef>,
}

impl From<&[Attribute<'_>]> for ModelUnits {
    fn from(value: &[Attribute<'_>]) -> Self {
        let hmap: HashMap<&str, &str> = value.iter().map(|a| (a.name(), a.value())).collect();
        ModelUnits {
            substance_units: hmap.get("subtance_units").map(|p| (*p).into()),
            time_units: hmap.get("time_units").map(|p| (*p).into()),
            extent_units: hmap.get("extent_units").map(|p| (*p).into()),
            volume_units: hmap.get("volume_units").map(|p| (*p).into()),
            area_units: hmap.get("area_units").map(|p| (*p).into()),
            length_units: hmap.get("length_units").map(|p| (*p).into()),
            conversion_factor: hmap.get("conversion_factor").map(|p| (*p).into()),
        }
    }
}
#[derive(Debug)]
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
#[derive(Debug)]
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
            units: value.attribute("units").map(|p| UnitSidRef::from(p)),
        }
    }
}
#[derive(Debug)]
pub struct Specie<'a> {
    compartment: &'a str,
    initial_concentration: Option<f64>,
    initial_amount: Option<f64>,
    substance_units: Option<UnitSidRef>,
    has_only_substance_units: bool,
    boundary_condition: bool,
    constant: bool,
    conversion_factor: Option<&'a str>,
}
impl<'a> From<Node<'a, 'a>> for Specie<'a> {
    fn from(value: Node<'a, 'a>) -> Self {
        Specie {
            compartment: value.attribute("compartment").unwrap(),
            initial_concentration: value
                .attribute("initialConcentration")
                .map(|p| p.parse().unwrap()),
            initial_amount: value.attribute("initialAmount").map(|p| p.parse().unwrap()),
            substance_units: value
                .attribute("substanceUnits")
                .map(|p| UnitSidRef::from(p)),
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
            conversion_factor: value.attribute("conversionFactor"),
        }
    }
}
#[derive(Debug)]
pub struct Parameter {
    value: Option<f64>,
    units: Option<UnitSidRef>,
    constant: bool,
}
impl<'a> From<Node<'a, 'a>> for Parameter {
    fn from(value: Node<'a, 'a>) -> Self {
        Parameter {
            value: value.attribute("value").map(|p| p.parse().unwrap()),
            units: value.attribute("units").map(|p| UnitSidRef::from(p)),
            constant: value.attribute("constant").unwrap().parse().unwrap(),
        }
    }
}
#[derive(Debug)]
pub struct InitialAssignment<'a> {
    pub symbol: &'a str,
}
type HL<'a, T> = HashMap<&'a str, T>;
#[derive(Debug)]
pub struct Model<'a> {
    pub model_units: ModelUnits,
    pub initial_assignments: HL<'a, InitialAssignment<'a>>,
    pub parameters: HL<'a, Parameter>,
    pub species: HL<'a, Specie<'a>>,
    pub compartments: HL<'a, Compartment>,
    pub unit_definitions: HL<'a, HashMap<UnitSId, Unit>>,
}

#[derive(Debug)]
pub struct Function<'a> {
    math: &'a str,
}
type Math<'a> = &'a str;
#[derive(Debug)]
pub enum Rule<'a> {
    AlgebraicRule { math: Math<'a> },
    AssignmentRule { math: Math<'a>, variable: &'a str },
    RateRule { math: Math<'a>, variable: &'a str },
}

use mathml::MathNode;
#[cfg(feature = "default")]
use pyo3::prelude::*;
use roxmltree::Attribute;
use roxmltree::Node;
use serde_derive::Deserialize;
use std::collections::HashMap;

fn unwrap_optional_str(value: Node<'_, '_>, attribute: &'_ str) -> Option<String> {
    match value.attribute(attribute) {
        Some(s) => Some(s.to_owned()),
        _ => None,
    }
}

fn unwrap_optional_ns(value: Node<'_, '_>, attribute: (&'_ str, &'_ str)) -> Option<String> {
    match value.attribute(attribute) {
        Some(s) => Some(s.to_owned()),
        _ => None,
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
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
#[derive(Debug, Hash, PartialEq, Eq, Deserialize, Clone)]
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
/// Metadata for models
///
/// # Example
///
/// ```
/// use roxmltree;
/// use rust_sbml::Annotation;
///
///let anotation: Annotation = roxmltree::Document::parse(
///     "<model extentUnits='substance' id='e_coli_core' metaid='e_coli_core' name='Escherichia coli str. K-12 substr. MG1655' substanceUnits='substance' timeUnits='time'>",
/// )
/// .unwrap()
/// .descendants()
/// .filter(|n| n.tag_name().name() == "model")
/// .map(|n| Annotation::from(n)).next().unwrap();
/// println!("{:?}", anotation);
/// assert_eq!(
///     anotation.name.unwrap(),
///     "Escherichia coli str. K-12 substr. MG1655".to_string()
/// );
/// ```
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Annotation {
    pub id: Option<String>,
    pub metaid: Option<String>,
    pub name: Option<String>,
}
impl From<Node<'_, '_>> for Annotation {
    fn from(value: Node<'_, '_>) -> Self {
        Annotation {
            id: unwrap_optional_str(value, "id"),
            metaid: unwrap_optional_str(value, "metaid"),
            name: unwrap_optional_str(value, "name"),
        }
    }
}

#[cfg_attr(feature = "default", pyclass)]
#[derive(Debug, PartialEq, Clone)]
pub struct Compartment {
    units: Option<UnitSidRef>,
    pub id: String,
    pub name: Option<String>,
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
            id: value.attribute("id").unwrap().to_owned(),
            name: unwrap_optional_str(value, "name"),
            size: value.attribute("size").map(|p| p.parse().unwrap()),
            constant: value.attribute("constant").unwrap().parse().unwrap(),
            units: value.attribute("units").map(|p| UnitSidRef::from(&p)),
        }
    }
}

#[cfg_attr(feature = "default", pyclass)]
#[derive(Debug, PartialEq, Clone)]
pub struct Specie {
    pub compartment: String,
    initial_concentration: Option<f64>,
    initial_amount: Option<f64>,
    pub id: String,
    substance_units: Option<UnitSidRef>,
    has_only_substance_units: bool,
    pub boundary_condition: bool,
    pub constant: bool,
    conversion_factor: Option<String>,
}
impl<'a> From<Node<'a, 'a>> for Specie {
    fn from(value: Node<'a, 'a>) -> Self {
        Specie {
            compartment: value.attribute("compartment").unwrap().to_owned(),
            id: value.attribute("id").unwrap().to_owned(),
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
#[cfg_attr(feature = "default", pyclass)]
#[derive(Debug, PartialEq, Clone)]
pub struct SpeciesReference {
    pub species: String,
    pub constant: bool,
    pub sbo_term: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub stoichiometry: Option<f64>,
}
impl<'a> From<Node<'a, 'a>> for SpeciesReference {
    fn from(value: Node<'a, 'a>) -> Self {
        SpeciesReference {
            species: value.attribute("species").unwrap().to_string(),
            constant: value.attribute("constant").unwrap().parse().unwrap(),
            sbo_term: unwrap_optional_str(value, "sboTerm"),
            id: unwrap_optional_str(value, "id"),
            name: unwrap_optional_str(value, "name"),
            stoichiometry: match value.attribute("stoichiometry") {
                Some(s) => Some(s.parse().unwrap()),
                None => None,
            },
        }
    }
}

#[cfg_attr(feature = "default", pyclass)]
#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub value: Option<f64>,
    units: Option<UnitSidRef>,
    pub constant: bool,
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
#[derive(Debug, PartialEq, Clone)]
pub struct ListOfSpecies(pub Vec<SpeciesReference>);
impl<'a> From<Node<'a, 'a>> for ListOfSpecies {
    fn from(value: Node<'a, 'a>) -> Self {
        ListOfSpecies(
            value
                .descendants()
                .filter(|n| n.tag_name().name() == "speciesReference")
                .map(SpeciesReference::from)
                .collect(),
        )
    }
}

/// Reaction object as defined by SBML
/// TODO: implement KineticLaw
///
/// # Example
///
/// ```
/// use roxmltree;
/// use rust_sbml::Reaction;
///
/// let reactions: Vec<Reaction> = roxmltree::Document::parse(
///     "<model id='example'><listOfReactions>
///         <reaction id='J1' reversible='false'>
///             <listOfReactants>
///                 <speciesReference species='X0' stoichiometry='2' constant='true'/>
///     </listOfReactants></reaction></listOfReactions></model>",
/// )
/// .unwrap()
/// .descendants()
/// .filter(|n| n.tag_name().name() == "reaction")
/// .map(|n| Reaction::from(n))
/// .collect();
/// println!("{:?}", reactions);
/// assert!(
///     reactions.iter().any(|reaction| reaction
///         .list_of_reactants
///         .0
///         .iter()
///         .any(|specref| specref.species == "X0"))
/// );
/// ```
#[cfg_attr(feature = "default", pyclass)]
#[derive(Debug, PartialEq, Clone)]
pub struct Reaction {
    pub id: String,
    pub list_of_reactants: ListOfSpecies,
    pub list_of_products: ListOfSpecies,
    pub reversible: bool,
    pub compartment: Option<String>,
    pub name: Option<String>,
    pub sbo_term: Option<String>,
    pub lower_bound: Option<String>,
    pub upper_bound: Option<String>,
}
impl<'a> From<Node<'a, 'a>> for Reaction {
    fn from(value: Node<'a, 'a>) -> Self {
        Reaction {
            id: value.attribute("id").unwrap().to_owned(),
            list_of_reactants: match value
                .children()
                .find(|n| n.tag_name().name() == "listOfReactants")
            {
                Some(n) => ListOfSpecies::from(n),
                _ => ListOfSpecies(Vec::new()),
            },
            list_of_products: match value
                .children()
                .find(|n| n.tag_name().name() == "listOfProducts")
            {
                Some(n) => ListOfSpecies::from(n),
                _ => ListOfSpecies(Vec::new()),
            },
            reversible: value.attribute("reversible").unwrap().parse().unwrap(),
            compartment: unwrap_optional_str(value, "compartment"),
            lower_bound: unwrap_optional_ns(
                value,
                (
                    "http://www.sbml.org/sbml/level3/version1/fbc/version2",
                    "lowerFluxBound",
                ),
            ),
            upper_bound: unwrap_optional_ns(
                value,
                (
                    "http://www.sbml.org/sbml/level3/version1/fbc/version2",
                    "upperFluxBound",
                ),
            ),
            name: unwrap_optional_str(value, "name"),
            sbo_term: unwrap_optional_str(value, "sboTerm"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Function {
    math: MathNode,
}
// #[derive(Debug, PartialEq)]
// pub enum Rule<'a> {
//     AlgebraicRule { math: MathNode },
//     AssignmentRule { math: MathNode, variable: &'a str },
//     RateRule { math: MathNode, variable: &'a str },
// }
#[derive(Debug, PartialEq)]
pub struct Constraint {
    pub math: Option<MathNode>,
    pub message: String,
}

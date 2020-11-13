// use mathml::MathNode;
#[cfg(feature = "default")]
use pyo3::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct UnitDefinition {
    pub id: Option<String>,
    #[serde(rename = "listOfUnits", default)]
    pub list_of_units: ListOfUnits,
}

#[derive(Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ListOfUnits {
    #[serde(rename = "unit")]
    pub units: Vec<Unit>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Unit {
    pub kind: UnitSIdRef,
    exponent: f64,
    scale: i64,
    multiplier: f64,
}

#[derive(Debug, Deserialize, Hash, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum UnitSIdRef {
    SIUnit(UnitSId),
    CustomUnit(String),
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

#[cfg_attr(feature = "default", pyclass)]
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Compartment {
    units: Option<UnitSIdRef>,
    pub id: String,
    pub name: Option<String>,
    spatial_dimensions: Option<f64>,
    size: Option<f64>,
    constant: bool,
}

/// A species in SBML refers to a pool of entities that
/// ⁻ are considered indistinguishable from each other for the purposes of the model;
/// - may participate in reactions;
/// - are located in a specific compartment.
#[cfg_attr(feature = "default", pyclass)]
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Species {
    pub id: String,
    pub compartment: String,
    initial_concentration: Option<f64>,
    initial_amount: Option<f64>,
    substance_units: Option<UnitSIdRef>,
    has_only_substance_units: bool,
    pub boundary_condition: bool,
    pub constant: bool,
    conversion_factor: Option<String>,
}

#[cfg_attr(feature = "default", pyclass)]
#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
pub struct Parameter {
    pub id: String,
    pub value: Option<f64>,
    units: Option<UnitSIdRef>,
    pub constant: bool,
}

/// InitialAssigments provide a way to compute initial values that must be
/// (using a MathML expression).
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct InitialAssignment {
    pub id: Option<String>,
    pub symbol: String,
    // pub math: Option<MathNode>,
    #[serde(rename = "sboTerm", default)]
    sbo_term: Option<String>,
}

#[cfg_attr(feature = "default", pyclass)]
#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct SpeciesReference {
    pub species: String,
    pub constant: bool,
    #[serde(rename = "sboTerm", default)]
    pub sbo_term: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub stoichiometry: Option<f64>,
}

#[derive(Debug, PartialEq, Clone, Default, Deserialize)]
pub struct ListOfSpeciesReferences {
    #[serde(rename = "speciesReference", default = "Vec::new")]
    pub species_references: Vec<SpeciesReference>,
}

/// A reaction in SBML represents any kind of process that can change the
/// quantity of one or more species in a model. Examples of such processes can
/// include transformation, transport, molecular interactions, and more.
/// TODO: implement KineticLaw
///
/// # Example
///
/// ```
/// use quick_xml::de::from_str;
/// use rust_sbml::Reaction;
///
/// let reactions: Reaction = from_str(
/// "<reaction id='J1' reversible='false' fbc:lowerFluxBound='-20'>
///         <listOfReactants>
///             <speciesReference species='X0' stoichiometry='2' constant='true'/>
/// </listOfReactants></reaction></listOfReactions></model>",
/// )
/// .unwrap();
/// println!("{:?}", reactions);
/// assert!(reactions
///     .list_of_reactants
///     .species_references
///     .iter()
///     .any(|specref| specref.species == "X0"));
/// ```
#[cfg_attr(feature = "default", pyclass)]
#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Reaction {
    pub id: String,
    #[serde(default)]
    pub list_of_reactants: ListOfSpeciesReferences,
    #[serde(default)]
    pub list_of_products: ListOfSpeciesReferences,
    pub reversible: bool,
    pub compartment: Option<String>,
    pub name: Option<String>,
    pub sbo_term: Option<String>,
    #[serde(rename = "fbc:lowerFluxBound")]
    pub lower_bound: Option<String>,
    #[serde(rename = "fbc:lowerUpperBound")]
    pub upper_bound: Option<String>,
}

// #[derive(Debug, PartialEq)]
// pub struct Function {
//     math: MathNode,
// }
// #[derive(Debug, PartialEq)]
// pub enum Rule<'a> {
//     AlgebraicRule { math: MathNode },
//     AssignmentRule { math: MathNode, variable: &'a str },
//     RateRule { math: MathNode, variable: &'a str },
// }
#[derive(Debug, Deserialize, PartialEq, Default, Clone)]
pub struct Constraint {
    // pub math: Option<MathNode>,
    pub message: String,
}

#[derive(Debug, Deserialize, PartialEq, Default, Clone)]
pub struct Objective {
    #[serde(rename = "fbc:id")]
    pub id: Option<String>,
    #[serde(rename = "fbc:type")]
    pub sense: Option<String>,
    #[serde(rename = "listOfFluxObjectives", default)]
    pub list_of_flux_objectives: ListOfFluxObjectives,
}

#[derive(Debug, Deserialize, PartialEq, Default, Clone)]
pub struct ListOfFluxObjectives {
    #[serde(rename="fluxObjective", default)]
    pub flux_objectives: Vec<FluxObjective>,
}

#[derive(Debug, Deserialize, PartialEq, Default, Clone)]
pub struct FluxObjective {
    #[serde(rename = "fbc:coefficient")]
    pub coefficient: Option<f64>,
    #[serde(rename = "fbc:reaction")]
    pub reaction: Option<String>,
}

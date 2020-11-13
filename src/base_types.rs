// use mathml::MathNode;
#[cfg(feature = "default")]
use pyo3::prelude::*;
use serde::Deserialize;

/// Combination of [`Unit`](./rust_sbml/struct.Unit.html).
///
/// The approach to defining units in SBML is compositional; for example,
/// metre second −2 is constructed by combining
/// an Unit object representing metre with another Unit object representing
/// second −2.
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

/// A Unit object represents a reference to a (possibly transformed) base unit
/// (see [UnitSIdRef](./rust_sbml/enum.UnitSIdRef.html).
///
/// The attribute kind indicates the base unit, whereas the attributes
/// exponent, scale and multiplier define how the base unit is being transformed.
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Unit {
    pub kind: UnitSIdRef,
    pub exponent: f64,
    pub scale: i64,
    pub multiplier: f64,
}

/// SBML provides predefined base units, gathered in [`UnitSId`](./rust_sbml/enum.UnitSId.html).
/// Alternatively, one can use arbitrary `CustomUnit`s.
#[derive(Debug, Deserialize, Hash, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum UnitSIdRef {
    SIUnit(UnitSId),
    CustomUnit(String),
}

/// One of the predefined values of a base unit by SBML level 3.
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

/// A compartment in SBML represents a bounded space in which species are located.
///
/// # Example
///
/// ```
/// use quick_xml::de::from_str;
/// use rust_sbml::Compartment;
///
/// let compartments: Vec<Compartment> = from_str(
///     "<compartment id='Extracellular' spatialDimensions='3' size='1e-14' constant='true'/>
///     <compartment id='PlasmaMembrane' spatialDimensions='2' size='1e-14' constant='true'/>
///     <compartment id='Cytosol' spatialDimensions='3' size='1e-15' constant='true'/>"
/// )
/// .unwrap();
/// assert!(compartments.iter()
///     .any(|c| c.spatial_dimensions.unwrap() as i32 == 2));
/// assert!(compartments.iter()
///     .any(|c| c.id == "Cytosol"));
/// assert!(compartments.iter()
///     .all(|c| c.constant));
/// ```
#[cfg_attr(feature = "default", pyclass)]
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Compartment {
    pub units: Option<UnitSIdRef>,
    pub id: String,
    pub name: Option<String>,
    pub spatial_dimensions: Option<f64>,
    pub size: Option<f64>,
    pub constant: bool,
}

/// A species in SBML refers to a pool of entities that
/// ⁻ are considered indistinguishable from each other for the purposes of the model;
/// - may participate in reactions;
/// - are located in a specific compartment.
///
/// # Example
///
/// ```
/// use quick_xml::de::from_str;
/// use rust_sbml::Species;
///
/// let species: Vec<Species> = from_str(
///     "<species id='Glucose' compartment='cell' initialConcentration='4'
///     hasOnlySubstanceUnits='false' boundaryCondition='false' constant='false'/>"
/// )
/// .unwrap();
/// assert_eq!(species[0].compartment, "cell");
/// assert_eq!(species[0].initial_concentration.unwrap() as u8, 4);
/// assert!(!species[0].constant);
/// ```
#[cfg_attr(feature = "default", pyclass)]
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Species {
    pub id: String,
    pub compartment: String,
    pub initial_concentration: Option<f64>,
    pub initial_amount: Option<f64>,
    pub substance_units: Option<UnitSIdRef>,
    pub has_only_substance_units: bool,
    pub boundary_condition: bool,
    pub constant: bool,
    pub conversion_factor: Option<String>,
}

/// A Parameter is used in SBML to define a symbol associated with a value;
/// this symbol can then be used in mathematical formulas in a model.
///
/// # Example
/// ```
/// use quick_xml::de::from_str;
/// use rust_sbml::{Parameter, UnitSIdRef, UnitSId};
///
/// let parameter: Vec<Parameter> = from_str(
///     "<parameter id=\"tau2\" value=\"3e-2\" units=\"second\" constant=\"true\"/>
///     <parameter id=\"Km1\" value=\"10.7\" units=\"molesperlitre\" constant=\"true\"/>"
/// )
/// .unwrap();
/// assert_eq!(
///     parameter[0].units.to_owned().unwrap(),
///     UnitSIdRef::SIUnit(UnitSId::Second)
/// );
/// assert_eq!(parameter[1].id, "Km1");
#[cfg_attr(feature = "default", pyclass)]
#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
pub struct Parameter {
    pub id: String,
    pub value: Option<f64>,
    pub units: Option<UnitSIdRef>,
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

/// Provide a way for reactions to define species as products and reactants.
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
///             <speciesReference species='X1' stoichiometry='1' constant='true'/>
/// </listOfReactants></reaction></listOfReactions></model>",
/// )
/// .unwrap();
/// println!("{:?}", reactions);
/// let mut specs_ref = reactions
///     .list_of_reactants
///     .species_references
///     .iter();
/// assert!(specs_ref
///     .any(|specref| specref.species == "X0"));
/// assert!(specs_ref
///     .all(|specref| specref.constant));
/// ```
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
///
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

/// TODO: MathML not integrated
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

/// The Constraint object is a mechanism for stating the assumptions under which
/// a model is designed to operate.
/// 
/// TODO: MathML not integrated
#[derive(Debug, Deserialize, PartialEq, Default, Clone)]
pub struct Constraint {
    // pub math: Option<MathNode>,
    pub message: String,
    #[serde(rename = "sboTerm")]
    pub sbo_term: Option<String>,
}

/// The Flux Balance Constraints package of SBML defines extensions for the 
/// model, including the FBC Objective. 
///
/// # Example
/// 
/// ```
/// use quick_xml::de::from_str;
/// use rust_sbml::Objective;
///
/// let objectives: Vec<Objective> = from_str(
/// "<fbc:objective fbc:id=\"obj1\" fbc:type=\"maximize\">
///     <fbc:listOfFluxObjectives>
///         <fbc:fluxObjective fbc:reaction=\"R101\" fbc:coefficient=\"1\"/>
///     </fbc:listOfFluxObjectives>
/// </fbc:objective>
/// <fbc:objective fbc:id=\"obj2\" fbc:type=\"minimize\">
///     <fbc:listOfFluxObjectives>
///         <fbc:fluxObjective fbc:reaction=\"R102\" fbc:coefficient=\"-2.5\"/>
///         <fbc:fluxObjective fbc:reaction=\"R103\" fbc:coefficient=\"1\"/>
///     </fbc:listOfFluxObjectives>
/// </fbc:objective>").unwrap();
///
/// objectives.iter().any(|o| o.sense == "maximize");
/// objectives[1].list_of_flux_objectives.flux_objectives.iter().any(|r| r.reaction.to_owned().unwrap() == "R103");
/// ```
#[derive(Debug, Deserialize, PartialEq, Default, Clone)]
pub struct Objective {
    #[serde(rename = "fbc:id")]
    pub id: String,
    #[serde(rename = "fbc:metaid")]
    pub metaid: Option<String>,
    #[serde(rename = "fbc:sboTerm")]
    pub sbo_term: Option<String>,
    #[serde(rename = "fbc:type")]
    pub sense: String,
    #[serde(rename = "listOfFluxObjectives", default)]
    pub list_of_flux_objectives: ListOfFluxObjectives,
}

#[derive(Debug, Deserialize, PartialEq, Default, Clone)]
pub struct ListOfFluxObjectives {
    #[serde(rename = "fluxObjective", default)]
    pub flux_objectives: Vec<FluxObjective>,
}

/// Relatively simple container for a model variable weighted by a signed
/// linear coefficient, defined in the Flux Balance Constraint package.
#[derive(Debug, Deserialize, PartialEq, Default, Clone)]
pub struct FluxObjective {
    #[serde(rename = "fbc:coefficient")]
    pub coefficient: Option<f64>,
    #[serde(rename = "fbc:reaction")]
    pub reaction: Option<String>,
}

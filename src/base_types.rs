// use mathml::MathNode;
use super::mathml::Math;
use super::UnitSIdRef;
#[cfg(feature = "default")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

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
/// assert!(compartments.iter() .any(|c| c.spatial_dimensions.unwrap() as i32 == 2));
/// assert!(compartments.iter()
///     .any(|c| c.id == "Cytosol"));
/// assert!(compartments.iter()
///     .all(|c| c.constant));
/// ```
#[cfg_attr(feature = "default", pyclass)]
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Compartment {
    pub units: Option<UnitSIdRef>,
    pub id: String,
    pub name: Option<String>,
    pub sbo_term: Option<String>,
    pub spatial_dimensions: Option<f64>,
    pub size: Option<f64>,
    pub constant: bool,
}

/// A species in SBML refers to a pool of entities that
///
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
/// assert_eq!(species[0].id, "Glucose");
/// assert_eq!(species[0].compartment, "cell");
/// assert_eq!(species[0].initial_concentration.unwrap() as u8, 4);
/// assert!(!species[0].constant);
/// assert!(!species[0].boundary_condition);
/// assert!(!species[0].has_only_substance_units);
/// ```
#[cfg_attr(feature = "default", pyclass)]
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Species {
    pub id: String,
    pub name: Option<String>,
    pub meta_id: Option<String>,
    pub sbo_term: Option<String>,
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
///     UnitSIdRef::SIUnit(UnitSId::second)
/// );
/// assert_eq!(parameter[1].id, "Km1");
#[cfg_attr(feature = "default", pyclass)]
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Default)]
pub struct Parameter {
    pub id: String,
    pub value: Option<f64>,
    pub units: Option<UnitSIdRef>,
    pub constant: bool,
}

/// InitialAssigments provide a way to declare initial values that must be
/// computed (using a MathML expression).
///
/// # Example
///
/// ```
/// use quick_xml::de::from_str;
/// use rust_sbml::{InitialAssignment, mathml::MathNode};
///
/// let initial_assg: InitialAssignment = from_str(
/// r#"<initialAssignment symbol="x">
///    <math xmlns="http://www.w3.org/1998/Math/MathML"
///    xmlns:sbml="http://www.sbml.org/sbml/level3/version2/core">
///    <apply>
///     <ci> y </ci>
///     <times/>
///     <cn sbml:units="dimensionless"> 2 </cn>
///    </apply>
///    </math>
///    </initialAssignment>
/// "#).unwrap();
/// assert_eq!(initial_assg.symbol, String::from("x"));
/// ```
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct InitialAssignment {
    pub id: Option<String>,
    pub symbol: String,
    pub math: Option<Math>,
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
///     .any(|specref| {println!("{:?}", specref); specref.stoichiometry.unwrap() as i32 == 1}));
/// assert!(specs_ref
///     .all(|specref| specref.constant));
/// ```
#[cfg_attr(feature = "default", pyclass)]
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct SpeciesReference {
    pub species: String,
    pub constant: bool,
    #[serde(rename = "sboTerm", default)]
    pub sbo_term: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub stoichiometry: Option<f64>,
}

#[derive(Debug, PartialEq, Clone, Default, Deserialize, Serialize)]
pub struct ListOfSpeciesReferences {
    #[serde(rename = "speciesReference", default = "Vec::new")]
    pub species_references: Vec<SpeciesReference>,
}

/// The [`KineticLaw`] object within a Reaction object can contain a
/// ListOfLocalParameters object containing the definitions of local parameter
/// that are only accessible within the scope of that particular reaction.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct LocalParameter {
    id: String,
    #[serde(rename = "sboTerm")]
    sbo_term: Option<String>,
    value: Option<f32>,
    units: UnitSIdRef,
}

#[derive(Debug, PartialEq, Clone, Default, Deserialize, Serialize)]
pub struct ListOfLocalParameters {
    #[serde(rename = "localParameter")]
    pub local_parameter: Vec<LocalParameter>,
}

/// The KineticLaw object class is used to describe the rate at which the
/// process defined by the Reaction takes place.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KineticLaw {
    math: Math,
    id: Option<String>,
    sbo_term: Option<String>,
    list_of_local_parameters: ListOfLocalParameters,
}

/// A reaction in SBML represents any kind of process that can change the
/// quantity of one or more species in a model. Examples of such processes can
/// include transformation, transport, molecular interactions, and more.
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
/// </listOfReactants>
/// <kineticLaw>
/// <math xmlns='http://www.w3.org/1998/Math/MathML'>
/// <apply>
/// <times/> <ci> k </ci> <ci> S2 </ci> <ci> X0 </ci> <ci> c1 </ci>
/// </apply>
/// </math>
/// <listOfLocalParameters>
/// <localParameter id='k' value='0.1' units='per_concent_per_time'/>
/// </listOfLocalParameters>
/// </kineticLaw></reaction>"
/// )
/// .unwrap();
///
/// println!("{:?}", reactions);
/// assert!(reactions.kinetic_law.is_some());
/// assert!(reactions
///     .list_of_reactants
///     .species_references
///     .iter()
///     .any(|specref| specref.species == "X0"));
/// ```
#[cfg_attr(feature = "default", pyclass)]
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Default)]
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
    pub kinetic_law: Option<KineticLaw>,
    #[serde(rename = "fbc:lowerFluxBound")]
    pub lower_bound: Option<String>,
    #[serde(rename = "fbc:lowerUpperBound")]
    pub upper_bound: Option<String>,
}

/// The FunctionDefinition object associates an identifier with a function
/// definition. This identifier can then be4 used as the function called in
/// subsequent MathML apply elements.
///
/// # Example
/// ```
/// use quick_xml::de::from_str;
/// use rust_sbml::FunctionDefinition;
///
/// let function: FunctionDefinition = from_str(
/// r#"<functionDefinition id="pow3">
///     <math xmlns="http://www.w3.org/1998/Math/MathML"
///     xmlns:sbml="http://www.sbml.org/sbml/level3/version2/core">
///         <lambda>
///             <bvar><ci> x </ci></bvar>
///             <apply> <power/> <ci> x </ci> <cn sbml:units="dimensionless"> 3 </cn>
///             </apply>
///         </lambda>
///     </math>
/// </functionDefinition>"#).unwrap();
///
/// assert_eq!(function.id, "pow3");
/// ```
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FunctionDefinition {
    #[serde(rename = "$value")]
    pub math: Math,
    pub id: String,
    pub sbo_term: Option<String>,
}

/// Rules provide additional ways to define the values of variables in a model, their
/// relationships, and the dynamical behaviors of those variables. Rules enable the encoding of
/// relationships that cannot be expressed using [`Reaction`]s alone nor by
/// an [`InitialAssignment`].
///
/// There are three function forms that involve a variable $x$; a function $f$;
/// $V$, a vector which does not include $x$; and $W$, a vector which may
/// include $x$.
///
/// # Example
///
/// The expression $k = \frac{k3}{k2}$ (from [the SBML3v2 spec](http://sbml.org/Special/specifications/sbml-level-3/version-2/core/release-2/sbml-level-3-version-2-release-2-core.pdf)):
///
/// ```
/// use quick_xml::de::from_str;
/// use rust_sbml::Rule;
///
/// let rule: Rule = from_str(
/// r#"<assignmentRule variable="k">
/// <math xmlns="http://www.w3.org/1998/Math/MathML">
/// <apply>
/// <divide/>
/// <ci> k3 </ci>
/// <ci> k2 </ci>
/// </apply>
/// </math>
/// </assignmentRule>"#).unwrap();
///
/// if let Rule::AssignmentRule { variable: k, .. } = rule {
///     assert_eq!(k.as_str(), "k");
/// } else {
///     panic!("Rule was not correctly parsed!")
/// }
/// ```
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Rule {
    /// $0 = f(W)$
    AlgebraicRule {
        #[serde(rename = "$value")]
        math: Math,
    },
    /// $x = f(V)$ (does not allow algebraic loops)
    AssignmentRule {
        #[serde(rename = "$value")]
        math: Math,
        variable: String,
    },
    /// $\frac{dx}{dt} = f(W)$
    RateRule {
        #[serde(rename = "$value")]
        math: Math,
        variable: String,
    },
}

/// A XML `<message>` node.
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename = "message")]
pub struct Message {
    #[serde(rename = "$value")]
    pub content: String,
}

/// The Constraint object is a mechanism for stating the assumptions under
/// which a model is designed to operate.
///
/// # Example
///
/// Constraint species “S1” so that $1 \le S1 \le 100$:
///
/// ```
/// use quick_xml::de::from_str;
/// use rust_sbml::Constraint;
///
/// let constraint: Constraint = from_str(
/// r#"<constraint>
///     <math xmlns="http://www.w3.org/1998/Math/MathML"
///             xmlns:sbml="http://www.sbml.org/sbml/level3/version2/core">
///         <apply>
///             <and/>
///             <apply> <lt/> <cn sbml:units="mole"> 1 </cn> <ci> S1 </ci>
///             </apply>
///             <apply> <lt/> <ci> S1 </ci> <cn sbml:units="mole"> 100 </cn>
///             </apply>
///         </apply>
///     </math>
///     <message>
///     <p xmlns="http://www.w3.org/1999/xhtml"> Species S1 is out of range. </p>
///     </message>
/// </constraint>"#).unwrap();
///
/// assert_eq!(constraint.message.unwrap().content.as_str(), "Species S1 is out of range.");
/// assert!(constraint.math.is_some())
/// ```
#[derive(Debug, Deserialize, Serialize, PartialEq, Default, Clone)]
pub struct Constraint {
    pub math: Option<Math>,
    pub message: Option<Message>,
    pub id: Option<String>,
    #[serde(rename = "sboTerm")]
    pub sbo_term: Option<String>,
}

/// The Flux Balance Constraints package of SBML defines extensions for the
/// model, including the FBC Objective.
///
/// See the [FBC specification](http://co.mbine.org/specifications/sbml.level-3.version-1.fbc.version-2.release-1.pdf)
/// for more details.
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
#[derive(Debug, Deserialize, Serialize, PartialEq, Default, Clone)]
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

#[derive(Debug, Deserialize, Serialize, PartialEq, Default, Clone)]
pub struct ListOfFluxObjectives {
    #[serde(rename = "fluxObjective", default)]
    pub flux_objectives: Vec<FluxObjective>,
}

/// Relatively simple container for a model variable weighted by a signed
/// linear coefficient, defined in the Flux Balance Constraint package.
#[derive(Debug, Deserialize, Serialize, PartialEq, Default, Clone)]
pub struct FluxObjective {
    #[serde(rename = "fbc:coefficient")]
    pub coefficient: Option<f64>,
    #[serde(rename = "fbc:reaction")]
    pub reaction: Option<String>,
}

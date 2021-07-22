#[cfg(feature = "default")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::base_types::{Compartment, Constraint, InitialAssignment, Parameter, Reaction, Species};
use super::list_of::*;
use super::{Unit, UnitSIdRef};

/// SBML model as defined in the [SBML Level 3 Version 2 core](http://sbml.org/Documents/Specifications).
///
/// Extended with [fbc plugin](http://sbml.org/Documents/Specifications/SBML_Level_3/Packages/fbc)
///
/// # Example
///
/// ```
/// use rust_sbml::ModelRaw;
/// use std::fs;
///
/// let ecoli = fs::read_to_string("examples/EcoliCore.xml").unwrap();
/// let raw_model = ModelRaw::parse(&ecoli).unwrap();
/// assert_eq!(
///     raw_model.list_of_unit_definitions.unit_definitions[0].id.to_owned().unwrap(),
///     "mmol_per_gDW_per_hr"
/// )
/// ```
#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Clone)]
#[serde(rename = "model", rename_all = "camelCase")]
pub struct ModelRaw {
    pub id: Option<String>,
    pub metaid: Option<String>,
    pub name: Option<String>,
    pub substance_units: Option<UnitSIdRef>,
    pub time_units: Option<UnitSIdRef>,
    pub extent_units: Option<UnitSIdRef>,
    pub volume_units: Option<UnitSIdRef>,
    pub area_units: Option<UnitSIdRef>,
    pub length_units: Option<UnitSIdRef>,
    pub conversion_factor: Option<UnitSIdRef>,
    #[serde(default)]
    pub list_of_unit_definitions: ListOfUnitDefinitions,
    #[serde(default)]
    pub list_of_compartments: ListOfCompartments,
    #[serde(default)]
    pub list_of_species: ListOfSpecies,
    #[serde(default)]
    pub list_of_parameters: ListOfParameters,
    #[serde(default)]
    pub list_of_initial_assignments: ListOfInitialAssignments,
    #[serde(default)]
    pub list_of_reactions: ListOfReactions,
    #[serde(default)]
    pub list_of_constraints: ListOfConstraints,
    #[serde(default)]
    pub list_of_objectives: Option<ListOfObjectives>,
    pub list_of_rules: Option<ListOfRules>,
    pub list_of_function_definitions: Option<ListOfFunctionDefinitions>,
}

impl ModelRaw {
    pub fn parse(doc: &str) -> Result<Self, quick_xml::DeError> {
        let raw_model: Sbml = quick_xml::de::from_str(doc)?;
        Ok(raw_model.model)
    }

    pub fn to_string(&self) -> Result<String, quick_xml::DeError> {
        quick_xml::se::to_string(&Sbml {
            model: (*self).clone(),
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
struct Sbml {
    model: ModelRaw,
}

/// Bucket struct to hold all units defined on the top level of
/// [`ModelRaw`].
#[derive(Debug, Default, PartialEq)]
pub struct ModelUnits {
    pub substance_units: Option<UnitSIdRef>,
    pub time_units: Option<UnitSIdRef>,
    pub extent_units: Option<UnitSIdRef>,
    pub volume_units: Option<UnitSIdRef>,
    pub area_units: Option<UnitSIdRef>,
    pub length_units: Option<UnitSIdRef>,
    pub conversion_factor: Option<UnitSIdRef>,
}

impl From<&ModelRaw> for ModelUnits {
    fn from(raw_model: &ModelRaw) -> Self {
        Self {
            substance_units: raw_model.substance_units.clone(),
            time_units: raw_model.time_units.clone(),
            extent_units: raw_model.extent_units.clone(),
            volume_units: raw_model.volume_units.clone(),
            area_units: raw_model.area_units.clone(),
            length_units: raw_model.length_units.clone(),
            conversion_factor: raw_model.conversion_factor.clone(),
        }
    }
}

type Hl<T> = HashMap<String, T>;
/// Abstraction over the SBML specification. It traverses each top-level
/// listOF_ and provides `HashMaps<id, object>` instead. In addition the model
/// units are gathered in an [`ModelUnits`] struct.
///
/// # Example
///
/// ```
/// use rust_sbml::Model;
/// use std::fs;
///
/// let ecoli = fs::read_to_string("examples/EcoliCore.xml").unwrap();
/// let document = Model::parse(&ecoli).unwrap();
/// println!("{:?}", document.objectives);
/// assert_eq!(
///     document
///         .objectives
///         .unwrap()
///         .iter()
///         .map(|reac_id| reac_id.to_owned())
///         .next(),
///     Some("R_BIOMASS_Ecoli_core_w_GAM".to_string())
/// );
/// ```
#[cfg_attr(feature = "default", pyclass)]
#[derive(Debug, Default, PartialEq)]
pub struct Model {
    pub id: Option<String>,
    pub metaid: Option<String>,
    pub name: Option<String>,
    pub model_units: ModelUnits,
    pub initial_assignments: Hl<InitialAssignment>,
    pub parameters: Hl<Parameter>,
    pub species: Hl<Species>,
    pub reactions: Hl<Reaction>,
    pub compartments: Hl<Compartment>,
    pub unit_definitions: Hl<HashMap<UnitSIdRef, Unit>>,
    pub constraints: Vec<Constraint>,
    pub objectives: Option<Vec<String>>,
}

impl Model {
    /// Emulating the API of libSBML
    ///
    /// # Example
    ///
    /// ```
    /// use rust_sbml::Model;
    /// use std::fs;
    ///
    /// let ecoli = fs::read_to_string("examples/EcoliCore.xml").unwrap();
    /// let document = Model::parse(&ecoli).unwrap();
    /// println!("{:?}", document.get_list_of_compartments())
    /// ```
    pub fn get_list_of_compartments(&self) -> Vec<&Compartment> {
        self.compartments.iter().map(|(_key, val)| val).collect()
    }
    /// Emulating the API of libSBML
    ///
    /// # Example
    ///
    /// ```
    /// use rust_sbml::Model;
    /// use std::fs;
    ///
    /// let ecoli = fs::read_to_string("examples/EcoliCore.xml").unwrap();
    /// let document = Model::parse(&ecoli).unwrap();
    /// println!("{:?}", document.get_list_of_species())
    /// ```
    pub fn get_list_of_species(&self) -> Vec<&Species> {
        self.species.iter().map(|(_key, val)| val).collect()
    }
    /// Emulating the API of libSBML
    ///
    /// # Example
    ///
    /// ```
    /// use rust_sbml::Model;
    /// use std::fs;
    ///
    /// let ecoli = fs::read_to_string("examples/EcoliCore.xml").unwrap();
    /// let document = Model::parse(&ecoli).unwrap();
    /// println!("{:?}", document.get_list_of_reactions())
    /// ```
    pub fn get_list_of_reactions(&self) -> Vec<&Reaction> {
        self.reactions.iter().map(|(_key, val)| val).collect()
    }
    /// Use [`ModelRaw`] to parse the SBML document
    /// and then format it into `Model`.
    pub fn parse(doc: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let raw_model = ModelRaw::parse(doc)?;
        // Units used by the model itself
        let model_units: ModelUnits = ModelUnits::from(&raw_model);

        // Unit definitions
        let unit_definitions: HashMap<String, HashMap<UnitSIdRef, Unit>> = raw_model
            .list_of_unit_definitions
            .unit_definitions
            .iter()
            .map(|unit_def| {
                (
                    (*unit_def).to_owned().id.unwrap(),
                    unit_def
                        .list_of_units
                        .units
                        .iter()
                        .map(|unit| (unit.kind.to_owned(), unit.to_owned()))
                        .collect(),
                )
            })
            .collect();
        // Compartments
        let compartments: HashMap<String, Compartment> = raw_model
            .list_of_compartments
            .compartments
            .iter()
            .map(|n| (n.id.to_owned(), n.to_owned()))
            .collect();
        // Species
        let species: HashMap<String, Species> = raw_model
            .list_of_species
            .species
            .iter()
            .map(|n| (n.id.to_owned(), n.to_owned()))
            .collect();
        // Parameters
        let parameters: HashMap<String, Parameter> = raw_model
            .list_of_parameters
            .parameters
            .iter()
            .map(|n| (n.id.to_owned(), n.to_owned()))
            .collect();
        // Initial assignments
        let initial_assignments: HashMap<String, InitialAssignment> = raw_model
            .list_of_initial_assignments
            .initial_assignments
            .iter()
            .map(|n| (n.id.to_owned().unwrap(), (*n).to_owned()))
            .collect();
        // Reactions
        let reactions: HashMap<String, Reaction> = raw_model
            .list_of_reactions
            .reactions
            .iter()
            .map(|n| (n.id.to_owned(), n.to_owned()))
            .collect();

        // Constraints
        let constraints: Vec<Constraint> = raw_model.list_of_constraints.constraints;
        let objectives: Option<Vec<String>> = match raw_model.list_of_objectives {
            Some(objs) => Some(
                objs.objectives
                    .iter()
                    .flat_map(|n| {
                        (*n).list_of_flux_objectives
                            .flux_objectives
                            .iter()
                            .map(|fr| fr.reaction.to_owned().unwrap())
                    })
                    .collect(),
            ),
            None => None,
        };
        Ok(Model {
            id: raw_model.id,
            metaid: raw_model.metaid,
            name: raw_model.name,
            model_units,
            parameters,
            initial_assignments,
            species,
            reactions,
            compartments,
            unit_definitions,
            constraints,
            objectives,
        })
    }
}

/// Shortcut to [`Model::parse`](Model::parse).
pub fn parse_document(doc: &str) -> Result<Model, Box<dyn std::error::Error>> {
    Model::parse(doc)
}

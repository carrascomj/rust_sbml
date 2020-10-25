#[cfg(feature = "default")]
use pyo3::prelude::*;
use std::collections::HashMap;

use super::base_types::{
    Annotation, Compartment, Constraint, InitialAssignment, ModelUnits, Parameter, Reaction,
    Specie, Unit, UnitSId,
};

type HL<T> = HashMap<String, T>;
/// Struct that holds the entire SBML document (non-comprehensive)
///
/// # Example
///
/// ```
/// use rust_sbml::Model;
/// use std::fs;
///
/// let ecoli = fs::read_to_string("examples/EcoliCore.xml").unwrap();
/// let document = Model::parse(&ecoli).unwrap();
/// assert_eq!(
///     document
///         .objectives
///         .iter()
///         .map(|reac_id| reac_id.to_owned())
///         .next()
///         .unwrap(),
///     "R_BIOMASS_Ecoli_core_w_GAM"
/// );
/// ```
#[cfg_attr(feature = "default", pyclass)]
#[derive(Debug, Default, PartialEq)]
pub struct Model {
    pub model_units: ModelUnits,
    pub initial_assignments: HL<InitialAssignment>,
    pub parameters: HL<Parameter>,
    pub species: HL<Specie>,
    pub reactions: HL<Reaction>,
    pub compartments: HL<Compartment>,
    pub unit_definitions: HL<HashMap<UnitSId, Unit>>,
    pub constraints: Vec<Constraint>,
    pub objectives: Vec<String>,
    pub annotation: Annotation,
}

impl Model {
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
    /// println!("{:?}", document.get_list_of_compartments())
    /// ```
    pub fn get_list_of_species(&self) -> Vec<&Specie> {
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
    /// println!("{:?}", document.get_list_of_species())
    /// ```
    pub fn get_list_of_reactions(&self) -> Vec<&Reaction> {
        self.reactions.iter().map(|(_key, val)| val).collect()
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
    pub fn parse(doc: &str) -> Result<Self, roxmltree::Error> {
        let res = roxmltree::Document::parse(doc)?;
        let raw_model = res
            .descendants()
            .find(|n| n.tag_name().name() == "model")
            .unwrap();

        // Units used by the model itself
        let model_units: ModelUnits = ModelUnits::from(raw_model.attributes());

        // Unit definitions
        let unit_definitions: HashMap<String, HashMap<UnitSId, Unit>> = raw_model
            .descendants()
            .filter(|n| n.tag_name().name() == "unitDefinition")
            .map(|r| {
                (
                    r.attribute("id").unwrap().to_owned(),
                    r.descendants()
                        .filter(|n| n.tag_name().name() == "unit")
                        .map(|r| {
                            (
                                r.attribute("kind")
                                    .map(serde_plain::from_str)
                                    .unwrap()
                                    .unwrap(),
                                Unit::from(r),
                            )
                        })
                        .collect(),
                )
            })
            .collect();
        let annotation: Annotation = raw_model
            .descendants()
            .filter(|n| n.tag_name().name() == "model")
            .map(Annotation::from)
            .next()
            .unwrap();
        // Compartments
        let compartments: HashMap<String, Compartment> = raw_model
            .descendants()
            .filter(|n| n.tag_name().name() == "compartment")
            .map(|n| (n.attribute("id").unwrap().to_owned(), Compartment::from(n)))
            .collect();
        // Species
        let species: HashMap<String, Specie> = raw_model
            .descendants()
            .filter(|n| n.tag_name().name() == "species")
            .map(|n| (n.attribute("id").unwrap().to_owned(), Specie::from(n)))
            .collect();
        // Parameters
        let parameters: HashMap<String, Parameter> = raw_model
            .descendants()
            .filter(|n| n.tag_name().name() == "parameter")
            .map(|n| (n.attribute("id").unwrap().to_owned(), Parameter::from(n)))
            .collect();
        // Initial assignments
        let initial_assignments: HashMap<String, InitialAssignment> = raw_model
            .descendants()
            .filter(|n| n.tag_name().name() == "initialAssignment")
            .map(|n| {
                (
                    n.attribute("id").unwrap().to_owned(),
                    InitialAssignment {
                        symbol: n.attribute("symbol").unwrap().to_owned(),
                    },
                )
            })
            .collect();
        // Reactions
        let reactions: HashMap<String, Reaction> = raw_model
            .descendants()
            .filter(|n| n.tag_name().name() == "reaction")
            .map(|n| (n.attribute("id").unwrap().to_owned(), Reaction::from(n)))
            .collect();

        // Constraints
        let constraints: Vec<Constraint> = raw_model
            .descendants()
            .filter(|n| n.tag_name().name() == "constraint")
            .map(|n| Constraint {
                math: n
                    .descendants()
                    .find(|n| n.tag_name().name() == "math")
                    .map(mathml::parse_node),
                message: n
                    .descendants()
                    .find(|n| n.tag_name().name() == "message")
                    .unwrap()
                    .children()
                    .map(|n| n.text().unwrap().trim().to_owned())
                    .collect::<String>(),
            })
            .collect();
        let objectives: Vec<String> = raw_model
            .descendants()
            .filter(|n| {
                n.tag_name()
                    == roxmltree::ExpandedName::from((
                        "http://www.sbml.org/sbml/level3/version1/fbc/version2",
                        "fluxObjective",
                    ))
            })
            .map(|n| {
                n.attribute((
                    "http://www.sbml.org/sbml/level3/version1/fbc/version2",
                    "reaction",
                ))
                .unwrap()
                .to_owned()
            })
            .collect();

        Ok(Model {
            model_units,
            parameters,
            initial_assignments,
            annotation,
            species,
            reactions,
            compartments,
            unit_definitions,
            constraints,
            objectives,
        })
    }
}

pub fn parse_document(doc: &str) -> Result<Model, roxmltree::Error> {
    Model::parse(doc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let reactions: Vec<Reaction> = roxmltree::Document::parse(
            "<model id='example'><listOfReactions>
                 <reaction id='J1' reversible='false'>
                     <listOfReactants>
                         <speciesReference species='X0' stoichiometry='2' constant='true'/>
                         <reaction id='J2' reversible='false'>
                             <listOfReactants>
                             <speciesReference species='CAP' stoichiometry='2' constant='true'/>
                             <speciesReference species='ZOOM' stoichiometry='-2' constant='true'/>
                     </listOfReactants></reaction>
             </listOfReactants></reaction></listOfReactions></model>",
        )
        .unwrap()
        .descendants()
        .filter(|n| n.tag_name().name() == "reaction")
        .map(Reaction::from)
        .collect();
        println!("{:?}", reactions);
        assert_eq!(reactions[1].list_of_reactants.0.len(), 2);
    }
}

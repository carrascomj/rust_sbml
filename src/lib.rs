mod base_types;
use std::collections::HashMap;

pub use base_types::{
    Compartment, Constraint, InitialAssignment, ModelUnits, Parameter, Reaction, Specie, Unit,
    UnitSId, UnitSidRef,
};

type HL<T> = HashMap<String, T>;
/// Struct that holds the entire SBML document (non-coprehensive)
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
                    .filter(|n| n.tag_name().name() == "math")
                    .next()
                    .map(mathml::parse_node),
                message: n
                    .descendants()
                    .filter(|n| n.tag_name().name() == "message")
                    .next()
                    .unwrap()
                    .children()
                    .map(|n| n.text().unwrap().trim().to_owned())
                    .collect::<String>(),
            })
            .collect();
        Ok(Model {
            model_units,
            parameters,
            initial_assignments,
            species,
            reactions,
            compartments,
            unit_definitions,
            constraints,
        })
    }
}

pub fn parse_document(doc: &str) -> Result<Model, roxmltree::Error> {
    Model::parse(doc)
}

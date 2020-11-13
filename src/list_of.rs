use serde::Deserialize;

use super::base_types::{
    Compartment, Constraint, InitialAssignment, Objective, Parameter, Reaction, Species,
    UnitDefinition,
};

#[derive(Deserialize, PartialEq, Debug, Default)]
pub struct ListOfUnitDefinitions {
    #[serde(rename = "unitDefinition", default)]
    pub unit_definitions: Vec<UnitDefinition>,
}

#[derive(Deserialize, PartialEq, Debug, Default)]
pub struct ListOfCompartments {
    #[serde(rename = "compartment", default)]
    pub compartments: Vec<Compartment>,
}

#[derive(Deserialize, PartialEq, Debug, Default)]
pub struct ListOfSpecies {
    pub species: Vec<Species>,
}

#[derive(Deserialize, PartialEq, Debug, Default)]
pub struct ListOfReactions {
    #[serde(rename = "reaction", default)]
    pub reactions: Vec<Reaction>,
}

#[derive(Deserialize, PartialEq, Debug, Default)]
pub struct ListOfParameters {
    #[serde(rename = "parameter", default)]
    pub parameters: Vec<Parameter>,
}

#[derive(Deserialize, PartialEq, Debug, Default)]
pub struct ListOfInitialAssignments {
    #[serde(rename = "initialAssigment", default)]
    pub initial_assignments: Vec<InitialAssignment>,
}

#[derive(Deserialize, PartialEq, Debug, Default)]
pub struct ListOfConstraints {
    #[serde(rename = "constraints", default)]
    pub constraints: Vec<Constraint>,
}

#[derive(Deserialize, PartialEq, Debug, Default)]
pub struct ListOfObjectives {
    #[serde(rename = "objective", default)]
    pub objectives: Vec<Objective>,
}

use serde::{Deserialize, Serialize};

use super::base_types::{
    Compartment, Constraint, FunctionDefinition, InitialAssignment, Objective, Parameter, Reaction,
    Rule, Species,
};
use super::unit::UnitDefinition;

#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct ListOfUnitDefinitions {
    #[serde(rename = "unitDefinition", default)]
    pub unit_definitions: Vec<UnitDefinition>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct ListOfCompartments {
    #[serde(rename = "compartment", default)]
    pub compartments: Vec<Compartment>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct ListOfSpecies {
    pub species: Vec<Species>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct ListOfReactions {
    #[serde(rename = "reaction", default)]
    pub reactions: Vec<Reaction>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct ListOfParameters {
    #[serde(rename = "parameter", default)]
    pub parameters: Vec<Parameter>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct ListOfInitialAssignments {
    #[serde(rename = "initialAssigment", default)]
    pub initial_assignments: Vec<InitialAssignment>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct ListOfConstraints {
    #[serde(rename = "constraint", default)]
    pub constraints: Vec<Constraint>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct ListOfObjectives {
    #[serde(rename = "objective", default)]
    pub objectives: Vec<Objective>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct ListOfRules {
    #[serde(rename = "$value", default)]
    pub rules: Vec<Rule>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct ListOfFunctionDefinitions {
    #[serde(rename = "functionDefinition", default)]
    pub function_definitions: Vec<FunctionDefinition>,
}

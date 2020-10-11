mod base_types;

pub use base_types::{
    Compartment, Constraint, InitialAssignment, Model, ModelUnits, Parameter, Reaction, Specie,
    Unit, UnitSId, UnitSidRef,
};
use std::collections::HashMap;

pub fn parse_document(doc: &str) -> Result<Model, roxmltree::Error> {
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
    // Initial assignments
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
                .nth(0)
                .map(mathml::parse_node),
            message: n
                .descendants()
                .filter(|n| n.tag_name().name() == "message")
                .nth(0)
                .unwrap()
                .children()
                .map(|n| n.text().unwrap().trim().to_owned())
                .collect::<String>(),
        })
        .collect();
    // Reactions
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

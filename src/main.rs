mod base_types;
use base_types::{
    Compartment, InitialAssignment, Model, ModelUnits, Parameter, Specie, Unit, UnitSId,
};
use std::collections::HashMap;
mod mathml;
fn main() {
    let doc = include_str!("rules.xml");
    let res = roxmltree::Document::parse(doc).unwrap();
    let raw_model = res
        .descendants()
        .find(|n| n.tag_name().name() == "model")
        .unwrap();
    // Units used by the model itself
    let model_units: ModelUnits = ModelUnits::from(raw_model.attributes());

    // Unit definitions
    let unit_definitions: HashMap<&str, HashMap<UnitSId, Unit>> = raw_model
        .descendants()
        .filter(|n| n.tag_name().name() == "unitDefinition")
        .map(|r| {
            (
                r.attribute("id").unwrap(),
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
    let compartments: HashMap<&str, Compartment> = raw_model
        .descendants()
        .filter(|n| n.tag_name().name() == "compartment")
        .map(|n| (n.attribute("id").unwrap(), Compartment::from(n)))
        .collect();
    // Species
    let species: HashMap<&str, Specie> = raw_model
        .descendants()
        .filter(|n| n.tag_name().name() == "species")
        .map(|n| (n.attribute("id").unwrap(), Specie::from(n)))
        .collect();
    // Parameters
    let parameters: HashMap<&str, Parameter> = raw_model
        .descendants()
        .filter(|n| n.tag_name().name() == "parameter")
        .map(|n| (n.attribute("id").unwrap(), Parameter::from(n)))
        .collect();
    // Initial assignments
    let initial_assignments: HashMap<&str, InitialAssignment> = raw_model
        .descendants()
        .filter(|n| n.tag_name().name() == "initialAssignment")
        .map(|n| {
            (
                n.attribute("id").unwrap(),
                InitialAssignment {
                    symbol: n.attribute("symbol").unwrap(),
                },
            )
        })
        .collect();

    // Reactions
    let _model = Model {
        model_units,
        parameters,
        initial_assignments,
        species,
        compartments,
        unit_definitions,
    };
    let math = r#"             <math xmlns="http://www.w3.org/1998/Math/MathML">
    <apply>
        <times/>
        <ci> k2 </ci>
        <ci> S2 </ci>
        <ci> cell </ci>
    </apply>
</math>"#;
    let pmath = roxmltree::Document::parse(math);
    println!("{:?}", pmath);
}

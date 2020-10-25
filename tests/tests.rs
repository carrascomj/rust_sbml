use mathml::BuiltinOp::*;
use mathml::MathNode::*;
use mathml::NumType::*;
use rust_sbml::{parse_document, Constraint, Model, ModelUnits, UnitSId, UnitSidRef};
use std::collections::HashMap;
#[test]
fn test_constraints() {
    let unit_type: HashMap<String, String> = vec![(
        "http://www.sbml.org/sbml/level3/version2/core:units".to_owned(),
        "mole".to_owned(),
    )]
    .into_iter()
    .collect();
    let example = include_str!("test_constraint.xml");
    let res = parse_document(example).unwrap();
    let expect = Model {
        model_units: ModelUnits {
            time_units: Some(UnitSidRef::SIUnit(UnitSId::Second)),
            substance_units: Some(UnitSidRef::SIUnit(UnitSId::Mole)),
            extent_units: Some(UnitSidRef::SIUnit(UnitSId::Mole)),
            volume_units: None,
            area_units: None,
            length_units: None,
            conversion_factor: None,
        },

        constraints: vec![Constraint {
            math: Some(Root(vec![Apply(vec![
                Op(and),
                Apply(vec![
                    Op(lt),
                    Cn {
                        num_type: Real(1.0),
                        base: 10,
                        definition_url: None,
                        encoding: None,
                        attributes: Some(unit_type.clone()),
                    },
                    Ci(vec![Text("S1".to_owned())]),
                ]),
                Apply(vec![
                    Op(lt),
                    Ci(vec![Text("S1".to_owned())]),
                    Cn {
                        num_type: Real(100.0),
                        base: 10,
                        definition_url: None,
                        encoding: None,
                        attributes: Some(unit_type),
                    },
                ]),
            ])])),
            message: "Species S1 is out of range.".to_owned(),
        }],
        ..Default::default()
    };
    assert_eq!(res, expect);
}
#[test]
fn test_simple() {
    let example = include_str!("test_simple.xml");
    let res = parse_document(example).unwrap();
    let expect = Model {
        model_units: ModelUnits {
            time_units: Some(UnitSidRef::SIUnit(UnitSId::Second)),
            substance_units: Some(UnitSidRef::SIUnit(UnitSId::Mole)),
            extent_units: Some(UnitSidRef::SIUnit(UnitSId::Mole)),
            ..Default::default()
        },
        ..Default::default()
    };
    assert_eq!(res, expect)
}

#[test]
fn from_genre() {
    let file_str = include_str!("EcoliCore.xml");
    let model = Model::parse(file_str).unwrap();
    assert_eq!(
        model
            .objectives
            .iter()
            .map(|reac_id| reac_id.to_owned())
            .next()
            .unwrap(),
        "R_BIOMASS_Ecoli_core_w_GAM"
    );
}

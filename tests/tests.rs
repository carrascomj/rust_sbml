use rust_sbml::{parse_document, Model, ModelRaw, ModelUnits, UnitSId, UnitSIdRef};

#[test]
fn read_units_succeed() {
    let example = include_str!("test_simple.xml");
    let res = parse_document(example);
    assert!(res.is_ok());
    let expect = Model {
        model_units: ModelUnits {
            time_units: Some(UnitSIdRef::SIUnit(UnitSId::second)),
            substance_units: Some(UnitSIdRef::SIUnit(UnitSId::mole)),
            extent_units: Some(UnitSIdRef::SIUnit(UnitSId::mole)),
            ..Default::default()
        },
        ..Default::default()
    };
    assert_eq!(res.unwrap(), expect)
}

#[test]
fn read_model_raw_objective_succeeds() {
    let file_str = include_str!("EcoliCore.xml");
    let model = ModelRaw::parse(file_str).unwrap();
    let gen_file_srt = model.to_string().unwrap();
    println!("{}", gen_file_srt);
    let model = Model::parse(&gen_file_srt).unwrap();

    assert_eq!(
        model
            .objectives
            .unwrap()
            .iter()
            .map(|reac_id| reac_id.to_owned())
            .next()
            .unwrap(),
        "R_BIOMASS_Ecoli_core_w_GAM"
    );
}

#[test]
fn read_abstraction_objective_succeeds() {
    let file_str = include_str!("EcoliCore.xml");
    let model = Model::parse(file_str).unwrap();
    assert_eq!(
        model
            .objectives
            .unwrap()
            .iter()
            .map(|reac_id| reac_id.to_owned())
            .next()
            .unwrap(),
        "R_BIOMASS_Ecoli_core_w_GAM"
    );
}

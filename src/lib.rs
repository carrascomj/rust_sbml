mod base_types;
mod model;
#[cfg(feature = "default")]
#[cfg(not(tarpaulin_include))]
mod pyo;

pub use base_types::{
    Annotation, Compartment, Constraint, InitialAssignment, ListOfSpecies, ModelUnits, Parameter,
    Reaction, Specie, SpeciesReference, Unit, UnitSId, UnitSidRef,
};

pub use model::{parse_document, Model};
#[cfg(feature = "default")]
pub use pyo::*;

#[cfg(test)]
mod tests {
    use super::*;
    use roxmltree;

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
        .map(|n| Reaction::from(n))
        .collect();
        println!("{:?}", reactions);
        assert_eq!(reactions[1].list_of_reactants.0.len(), 2);
    }
}

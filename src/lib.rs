//! Parser for the [Systems Biology Markup Language (SBML)](http://sbml.org/Special/specifications/sbml-level-3/version-2/core/release-2/sbml-level-3-version-2-release-2-core.pdf):
//!   * [Standalone Rust library](#rust)
//!   * [Python API](#python)
//!
//! # Getting started
//!
//! ## Rust
//! Add it to your Cargo.toml with no default features to avoid all
//! [PyO3](https://github.com/PyO3/pyo3) nuisances.
//!
//! ```toml
//! [dependencies.rust_sbml]
//! version = "0.6.0"
//! default_features=false
//! ```
//!
//! For example,
//!
//! ```rust
//! use rust_sbml::Model;
//!
//! let example=r#"<?xml version="1.0" encoding="UTF-8"?>
//! <sbml xmlns="http://www.sbml.org/sbml/level3/version2/core" level="3" version="2">
//!      <model timeUnits="second" extentUnits="mole" substanceUnits="mole">
//!      </model>
//! </sbml>"#;
//! let result = Model::parse(example);
//! println!("{:?}", result.unwrap());
//! ```
//!
//! See [write_to_file.rs](https://github.com/carrascomj/rust_sbml/blob/trunk/examples/write_to_file.rs)
//! for an example on serializing to a file.
//!
//! ## Python
//! It has only been tested on Linux.
//! ### Using pip
//!
//! ```shell
//! pip install rust_sbml
//! ```
//!
//! ### From source
//! Clone the repository.
//! ```shell
//! git clone https://github.com/carrascomj/rust_sbml.git
//! ```
//! You need [maturin](https://github.com/PyO3/maturin) for building it.
//! ```shell
//! python -m pip install maturin
//! ```
//! * Build locally
//!   ```shell
//!   maturin build --release
//!   pip install .
//!   ```
//! * Build on virtualenv (no pip install required)
//!   ```shell
//!   # --release can be omitted to speed up compilation time
//!   maturin develop --release
//!   ```
//!
//! Having it installed, you can use it as a normal Python package.
//!
//! ```python
//! from rust_sbml import Model
//!
//! sbml = Model("examples/EcoliCore.xml")
//! reaction = sbml.getListOfReactions()[0]
//! print(reaction.getListOfReactants())
//! ```
//!
//! ## Milestones
//! * `getListOfSpecies()` (id, name)
//! * `getListOfCompartments()` (id, name)
//! * `getListOfReactions()` (id, name)
//!   * `.getListOfReactants()` (id, name)
//!   * .`getListOfProducts()` (id, name)
//! * Capable of retrieving FBC bounds.
//! * Published to pypi
//! * Kinetic Laws.
//! * _(Missing) Metadata_.
//! * Test suite with python calls.
//! * Test suite with libsbml comparison trough cobrapy.
mod base_types;
mod list_of;
pub mod mathml;
mod model;
#[cfg(feature = "default")]
mod pyo;
mod unit;

pub use base_types::{
    Compartment, Constraint, FunctionDefinition, InitialAssignment, Message, Objective, Parameter,
    Reaction, Rule, Species, SpeciesReference,
};
pub use unit::{Unit, UnitSId, UnitSIdRef};

pub use model::{parse_document, Model, ModelRaw, ModelUnits};
#[cfg(feature = "default")]
pub use pyo::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_model_with_reactions() {
        let reactions: Result<Vec<Reaction>, quick_xml::DeError> = quick_xml::de::from_str(
            "<reaction id='J1' reversible='false'>
             <listOfReactants>
                     <speciesReference species='X0' stoichiometry='2' constant='true'/>
                 </listOfReactants>
              </reaction>
             <reaction id='J2' reversible='false'>
                 <listOfReactants>
                 <speciesReference species='CAP' stoichiometry='2' constant='true'/>
                 <speciesReference species='ZOOM' stoichiometry='-2' constant='true'/>
                 </listOfReactants>
             </reaction>",
        );
        assert!(reactions.is_ok());
        let reactions = reactions.unwrap();
        println!("{:?}", reactions);
        assert_eq!(reactions[1].list_of_reactants.species_references.len(), 2);
    }
}

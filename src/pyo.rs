#![allow(non_snake_case)]

use super::{Compartment, Model, Parameter, Reaction, Species, SpeciesReference};
use pyo3::prelude::*;

#[pymethods]
impl Species {
    #[getter]
    fn id(&self) -> &str {
        self.id.as_str()
    }
    fn getCompartment(&self) -> &str {
        self.compartment.as_str()
    }
}

#[pymethods]
impl Compartment {
    #[getter]
    fn id(&self) -> &str {
        self.id.as_str()
    }
    #[getter]
    fn name(&self) -> &str {
        match &self.name {
            Some(s) => s,
            _ => "",
        }
    }
}

#[pymethods]
impl Reaction {
    #[getter]
    fn id(&self) -> &str {
        self.id.as_str()
    }
    #[getter]
    fn name(&self) -> &str {
        match &self.name {
            Some(s) => s,
            _ => "",
        }
    }
    fn getListOfReactants(&self) -> Vec<SpeciesReference> {
        self.list_of_reactants.species_references.to_owned()
    }
    fn getListOfProducts(&self) -> Vec<SpeciesReference> {
        self.list_of_products.species_references.to_owned()
    }
    fn getLowerFluxBound(&self) -> &str {
        match &self.lower_bound {
            Some(s) => s,
            _ => "",
        }
    }
    fn getUpperFluxBound(&self) -> &str {
        match &self.upper_bound {
            Some(s) => s,
            None => "",
        }
    }
}

#[pymethods]
impl SpeciesReference {
    #[getter]
    fn id(&self) -> &str {
        self.species.as_str()
    }
    fn getStoichiometry(&self) -> Option<f64> {
        self.stoichiometry
    }
}

#[pymethods]
impl Parameter {
    fn getValue(&self) -> Option<f64> {
        self.value
    }
    fn getConstant(&self) -> bool {
        self.constant
    }
}

#[pymethods]
impl Model {
    #[new]
    fn new(doc: &str) -> Self {
        let file_str = std::fs::read_to_string(doc).unwrap();
        match Model::parse(&file_str) {
            Ok(m) => m,
            Err(e) => panic!("kai_sbml Couldn't parse {}. Error: {:?}", doc, e),
        }
    }
    fn getListOfCompartments(&self) -> Vec<Compartment> {
        self.compartments
            .iter()
            .map(|(_, n)| n.to_owned())
            .collect()
    }
    fn getListOfSpecies(&self) -> Vec<Species> {
        self.species.iter().map(|(_, n)| n.to_owned()).collect()
    }
    fn getListOfReactions(&self) -> Vec<Reaction> {
        self.reactions.iter().map(|(_, n)| n.to_owned()).collect()
    }
    fn getParameter(&self, query: String) -> Option<Parameter> {
        self.parameters.get(&query).cloned()
    }
    fn getObjectives(&self) -> Vec<String> {
        self.objectives.to_owned().unwrap_or_default()
    }
    #[getter]
    fn id(&self) -> Option<String> {
        self.id.to_owned()
    }
    #[getter]
    fn metaid(&self) -> Option<String> {
        self.metaid.to_owned()
    }
    #[getter]
    fn name(&self) -> Option<String> {
        self.name.to_owned()
    }
}

#[pymodule]
fn rust_sbml(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Model>()?;
    m.add_class::<Reaction>()?;
    m.add_class::<Species>()?;
    m.add_class::<SpeciesReference>()?;
    m.add_class::<Compartment>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

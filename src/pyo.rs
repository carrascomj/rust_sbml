#![allow(non_snake_case)]
use super::{Compartment, Model, Parameter, Reaction, Specie, SpeciesReference};
use pyo3::prelude::*;

#[pymethods]
impl Specie {
    #[getter]
    fn id(&self) -> PyResult<String> {
        Ok(self.id.to_owned())
    }
    fn getCompartment(&self) -> PyResult<String> {
        Ok(self.compartment.to_owned())
    }
}

#[pymethods]
impl Compartment {
    #[getter]
    fn id(&self) -> PyResult<String> {
        Ok(self.id.to_owned())
    }
    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(match self.name.to_owned() {
            Some(s) => s,
            _ => "".to_owned(),
        })
    }
}

#[pymethods]
impl Reaction {
    #[getter]
    fn id(&self) -> PyResult<String> {
        Ok(self.id.to_owned())
    }
    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(match self.name.to_owned() {
            Some(s) => s,
            _ => "".to_string(),
        })
    }
    fn getListOfReactants(&self) -> PyResult<Vec<SpeciesReference>> {
        Ok(self.list_of_reactants.to_owned().0)
    }
    fn getListOfProducts(&self) -> PyResult<Vec<SpeciesReference>> {
        Ok(self.list_of_products.to_owned().0)
    }
    fn getLowerFluxBound(&self) -> PyResult<String> {
        Ok(match self.lower_bound.to_owned() {
            Some(s) => s,
            _ => "".to_string(),
        })
    }
    fn getUpperFluxBound(&self) -> PyResult<String> {
        Ok(match self.upper_bound.to_owned() {
            Some(s) => s,
            _ => "".to_string(),
        })
    }
}

#[pymethods]
impl SpeciesReference {
    #[getter]
    fn id(&self) -> PyResult<String> {
        Ok(self.species.to_owned())
    }
    fn getStoichiometry(&self) -> PyResult<f64> {
        Ok(match self.stoichiometry {
            Some(s) => s,
            _ => 1.,
        })
    }
}

#[pymethods]
impl Parameter {
    fn getValue(&self) -> PyResult<f64> {
        Ok(match self.value {
            Some(p) => p,
            _ => panic!("no value"),
        })
    }
    fn getConstant(&self) -> PyResult<bool> {
        Ok(self.constant)
    }
}

#[pymethods]
impl Model {
    #[new]
    fn new(doc: String) -> Self {
        let file_str = std::fs::read_to_string(&doc).unwrap();
        match Model::parse(&file_str) {
            Ok(m) => m,
            Err(e) => panic!("kai_sbml Couldn't parse {}. Error: {:?}", doc, e),
        }
    }
    fn getListOfCompartments(&self) -> PyResult<Vec<Compartment>> {
        Ok(self
            .compartments
            .iter()
            .map(|(_, n)| n.to_owned())
            .collect())
    }
    fn getListOfSpecies(&self) -> PyResult<Vec<Specie>> {
        Ok(self.species.iter().map(|(_, n)| n.to_owned()).collect())
    }
    fn getListOfReactions(&self) -> PyResult<Vec<Reaction>> {
        Ok(self.reactions.iter().map(|(_, n)| n.to_owned()).collect())
    }
    fn getParameter(&self, query: String) -> PyResult<Parameter> {
        Ok(self.parameters[&query].to_owned())
    }
    fn getObjectives(&self) -> PyResult<Vec<String>> {
        Ok(self.objectives.to_owned())
    }
    #[getter]
    fn id(&self) -> PyResult<String> {
        Ok(match self.annotation.id.to_owned() {
            Some(s) => s,
            None => "".to_string(),
        })
    }
    #[getter]
    fn metaid(&self) -> PyResult<String> {
        Ok(match self.annotation.metaid.to_owned() {
            Some(s) => s,
            None => "".to_string(),
        })
    }
    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(match self.annotation.name.to_owned() {
            Some(s) => s,
            None => "".to_string(),
        })
    }
}

#[pymodule]
fn rust_sbml(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Model>()?;
    m.add_class::<Reaction>()?;
    m.add_class::<Specie>()?;
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

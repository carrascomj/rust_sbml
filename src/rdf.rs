use serde::{Deserialize, Serialize};

/// Rdf element from [xmlns:rdf](https://www.w3.org/TR/2014/REC-rdf-syntax-grammar-20140225/), tailored for BioModels.
/// This is used in SBML to store annotations of [`crate::base_types::Species`], [`crate::base_types::Reaction`] and the [`crate::Model`]
///
/// TODO: missing intermediate rdf:Description
///
/// # Example
///
/// ```
/// use quick_xml::de::from_str;
/// use rust_sbml::Species;
/// use std::convert::TryInto;
/// use std::collections::HashMap;
///
/// let species: Vec<Species> = from_str(
///     "<species id='Glucose' compartment='cell' initialConcentration='4'
///     hasOnlySubstanceUnits='false' boundaryCondition='false' constant='false'>
///     <annotation>
///     <rdf:RDF>
///         <rdf:Description>
///         <bqbiol:is>
///             <rdf:Bag>
///                  <rdf:li rdf:resource='someuri.com/bigg/h'/>
///                  <rdf:li rdf:resource='someuri.com/seed/cpd00067'/>
///                  <rdf:li rdf:resource='someuri.com/mnx/MNX00004'/>
///             </rdf:Bag>
///         </bqbiol:is>
///         </rdf:Description>
///     </rdf:RDF>
///     </annotation>
///     </species>"
/// )
/// .unwrap();
/// assert_eq!(species[0].annotation.as_ref().unwrap().flatten().unwrap().len(), 3);
/// let annot: HashMap<String, String> = species[0].annotation.as_ref().unwrap().into();
/// assert_eq!(annot["bigg"], "h")
/// ```
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Default)]
pub struct Rdf {
    #[serde(rename = "$unflatten=rdf:Description", default)]
    pub description: RdfDescriptor,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Default)]
pub struct RdfDescriptor {
    #[serde(rename = "$unflatten=bqbiol:is", default)]
    pub inner: Vec<Bqbiol>,
}

/// TODO: requires custom Deserialize, Serialize. Right now, it is only parsing Is
///
/// # Example
///
/// ```xml
/// <rdf:RDF>
///     <bqbiol:is>
///         <rdf:li rdf:resource="someuri.com">
///         <rdf:li rdf:resource="someuri.com">
///         <rdf:li rdf:resource="someuri.com">
///     </bqbiol:is>
/// </rdf:RDF>
/// ```
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Bqbiol {
    #[serde(skip, default)]
    pub bq_type: BqbiolType,
    #[serde(rename = "$unflatten=rdf:Bag", default)]
    pub rdf_bag: RdfBag,
}

/// Model qualifier, see <http://co.mbine.org/standards/qualifiers>
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum BqbiolType {
    Encodes,
    HasPart,
    HasProperty,
    HasVersion,
    Is,
    IsDescribedBy,
    IsEncodedBy,
    IsHomologTo,
    IsPartOf,
    IsPropertyOf,
    IsVersionOf,
    OccursIn,
    HasTaxon,
}

impl Default for BqbiolType {
    fn default() -> Self {
        BqbiolType::Is
    }
}

/// Container of [`rdf:li`](`RdfLi`).
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Default)]
pub struct RdfBag {
    #[serde(rename = "$unflatten=rdf:li", default)]
    pub rdf_lis: Vec<RdfLi>,
}

/// An element in a [`rdf:Bag`](`RdfBag`) with a resource.
#[derive(Debug, PartialEq, Clone, Default, Deserialize, Serialize)]
pub struct RdfLi {
    #[serde(rename = "rdf:resource")]
    pub resource: String,
}

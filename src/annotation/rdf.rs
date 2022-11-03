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
/// let annot: HashMap<&str, Vec<&str>> = species[0].annotation.as_ref().unwrap().into();
/// assert_eq!(annot["bigg"][0], "h")
/// ```
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Default)]
pub struct Rdf {
    #[serde(rename = "$unflatten=rdf:Description", default)]
    pub description: RdfDescriptor,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Default)]
pub struct RdfDescriptor {
    #[serde(rename = "$value", default)]
    pub inner: Vec<Bqbiol>,
}

/// A heavily-hardcoded model qualifier enum for `<bqbiol:VARIANT>`.
///
/// See <http://co.mbine.org/standards/qualifiers>
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
/// }
///
/// # Warnings
///
/// Some documents may use namespaces not covered by `[rust_sbml::annotation::Annotation]`.
/// To avoid deserialization errors, those are gathered into `[Bqbiol::Unit]` (which
/// is a bit hacky).
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub enum Bqbiol {
    #[serde(rename = "bqbiol:encodes")]
    Encodes {
        #[serde(rename = "$unflatten=rdf:Bag", default)]
        rdf_bag: RdfBag,
    },
    #[serde(rename = "bqbiol:hasPart")]
    HasPart {
        #[serde(rename = "$unflatten=rdf:Bag", default)]
        rdf_bag: RdfBag,
    },
    #[serde(rename = "bqbiol:encodes")]
    HasProperty {
        #[serde(rename = "$unflatten=rdf:Bag", default)]
        rdf_bag: RdfBag,
    },
    #[serde(rename = "bqbiol:hasVersion")]
    HasVersion {
        #[serde(rename = "$unflatten=rdf:Bag", default)]
        rdf_bag: RdfBag,
    },
    #[serde(rename = "bqbiol:is")]
    Is {
        #[serde(rename = "$unflatten=rdf:Bag", default)]
        rdf_bag: RdfBag,
    },
    #[serde(rename = "bqbiol:isDescribedBy")]
    IsDescribedBy {
        #[serde(rename = "$unflatten=rdf:Bag", default)]
        rdf_bag: RdfBag,
    },
    #[serde(rename = "bqbiol:isEncodedBy")]
    IsEncodedBy {
        #[serde(rename = "$unflatten=rdf:Bag", default)]
        rdf_bag: RdfBag,
    },
    #[serde(rename = "bqbiol:isHomologTo")]
    IsHomologTo {
        #[serde(rename = "$unflatten=rdf:Bag", default)]
        rdf_bag: RdfBag,
    },
    #[serde(rename = "bqbiol:isPartOf")]
    IsPartOf {
        #[serde(rename = "$unflatten=rdf:Bag", default)]
        rdf_bag: RdfBag,
    },
    #[serde(rename = "bqbiol:isPropertyOf")]
    IsPropertyOf {
        #[serde(rename = "$unflatten=rdf:Bag", default)]
        rdf_bag: RdfBag,
    },
    #[serde(rename = "bqbiol:isVersionOf")]
    IsVersionOf {
        #[serde(rename = "$unflatten=rdf:Bag", default)]
        rdf_bag: RdfBag,
    },
    #[serde(rename = "bqbiol:occursIn")]
    OccursIn {
        #[serde(rename = "$unflatten=rdf:Bag", default)]
        rdf_bag: RdfBag,
    },
    #[serde(rename = "bqbiol:hasTaxon")]
    HasTaxon {
        #[serde(rename = "$unflatten=rdf:Bag", default)]
        rdf_bag: RdfBag,
    },
    #[serde(rename = "bqmodel:encodes")]
    ModelEncodes {
        #[serde(rename = "$unflatten=rdf:Bag", default)]
        rdf_bag: RdfBag,
    },
    #[serde(rename = "bqmodel:hasPart")]
    ModelHasPart {
        #[serde(rename = "$unflatten=rdf:Bag", default)]
        rdf_bag: RdfBag,
    },
    #[serde(rename = "bqmodel:encodes")]
    ModelHasProperty {
        #[serde(rename = "$unflatten=rdf:Bag", default)]
        rdf_bag: RdfBag,
    },
    #[serde(rename = "bqmodel:hasVersion")]
    ModelHasVersion {
        #[serde(rename = "$unflatten=rdf:Bag", default)]
        rdf_bag: RdfBag,
    },
    #[serde(rename = "bqmodel:is")]
    ModelIs {
        #[serde(rename = "$unflatten=rdf:Bag", default)]
        rdf_bag: RdfBag,
    },
    #[serde(rename = "bqmodel:isDescribedBy")]
    ModelIsDescribedBy {
        #[serde(rename = "$unflatten=rdf:Bag", default)]
        rdf_bag: RdfBag,
    },
    #[serde(rename = "bqmodel:isEncodedBy")]
    ModelIsEncodedBy {
        #[serde(rename = "$unflatten=rdf:Bag", default)]
        rdf_bag: RdfBag,
    },
    #[serde(rename = "bqmodel:isHomologTo")]
    ModelIsHomologTo {
        #[serde(rename = "$unflatten=rdf:Bag", default)]
        rdf_bag: RdfBag,
    },
    #[serde(rename = "bqmodel:isPartOf")]
    ModelIsPartOf {
        #[serde(rename = "$unflatten=rdf:Bag", default)]
        rdf_bag: RdfBag,
    },
    #[serde(rename = "bqmodel:isPropertyOf")]
    ModelIsPropertyOf {
        #[serde(rename = "$unflatten=rdf:Bag", default)]
        rdf_bag: RdfBag,
    },
    #[serde(rename = "bqmodel:isVersionOf")]
    ModelIsVersionOf {
        #[serde(rename = "$unflatten=rdf:Bag", default)]
        rdf_bag: RdfBag,
    },
    #[serde(rename = "bqmodel:occursIn")]
    ModelOccursIn {
        #[serde(rename = "$unflatten=rdf:Bag", default)]
        rdf_bag: RdfBag,
    },
    #[serde(rename = "bqmodel:hasTaxon")]
    ModelHasTaxon {
        #[serde(rename = "$unflatten=rdf:Bag", default)]
        rdf_bag: RdfBag,
    },
    /// This is to allow any not covered annotation like dcterms
    #[serde(other)]
    Other,
}

impl Bqbiol {
    pub fn bag(&self) -> Option<&RdfBag> {
        match self {
            Bqbiol::Encodes { rdf_bag: x } => Some(x),
            Bqbiol::HasPart { rdf_bag: x } => Some(x),
            Bqbiol::HasProperty { rdf_bag: x } => Some(x),
            Bqbiol::HasVersion { rdf_bag: x } => Some(x),
            Bqbiol::Is { rdf_bag: x } => Some(x),
            Bqbiol::IsDescribedBy { rdf_bag: x } => Some(x),
            Bqbiol::IsEncodedBy { rdf_bag: x } => Some(x),
            Bqbiol::IsHomologTo { rdf_bag: x } => Some(x),
            Bqbiol::IsPartOf { rdf_bag: x } => Some(x),
            Bqbiol::IsPropertyOf { rdf_bag: x } => Some(x),
            Bqbiol::IsVersionOf { rdf_bag: x } => Some(x),
            Bqbiol::OccursIn { rdf_bag: x } => Some(x),
            Bqbiol::HasTaxon { rdf_bag: x } => Some(x),
            Bqbiol::ModelEncodes { rdf_bag: x } => Some(x),
            Bqbiol::ModelHasPart { rdf_bag: x } => Some(x),
            Bqbiol::ModelHasProperty { rdf_bag: x } => Some(x),
            Bqbiol::ModelHasVersion { rdf_bag: x } => Some(x),
            Bqbiol::ModelIs { rdf_bag: x } => Some(x),
            Bqbiol::ModelIsDescribedBy { rdf_bag: x } => Some(x),
            Bqbiol::ModelIsEncodedBy { rdf_bag: x } => Some(x),
            Bqbiol::ModelIsHomologTo { rdf_bag: x } => Some(x),
            Bqbiol::ModelIsPartOf { rdf_bag: x } => Some(x),
            Bqbiol::ModelIsPropertyOf { rdf_bag: x } => Some(x),
            Bqbiol::ModelIsVersionOf { rdf_bag: x } => Some(x),
            Bqbiol::ModelOccursIn { rdf_bag: x } => Some(x),
            Bqbiol::ModelHasTaxon { rdf_bag: x } => Some(x),
            _ => None,
        }
    }
}

impl Default for Bqbiol {
    fn default() -> Self {
        Bqbiol::Is {
            rdf_bag: RdfBag::default(),
        }
    }
}

/// Container of [`rdf:li`](`RdfLi`).
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Default)]
pub struct RdfBag {
    #[serde(rename = "$unflatten=rdf:li", default)]
    pub rdf_lis: Vec<RdfLi>,
}

/// An element in a [`rdf:Bag`](`RdfBag`) with a resource.
#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
pub struct RdfLi {
    #[serde(rename = "rdf:resource")]
    pub resource: String,
}

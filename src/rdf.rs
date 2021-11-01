use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::From};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
    #[serde(rename = "$unflatten=rdf:RDF")]
    pub rdf: Option<Rdf>,
}

impl Annotation {
    pub fn flatten(&self) -> Option<Vec<&str>> {
        self.rdf.as_ref().map(|rdf| {
            rdf.description
                .inner
                .iter()
                .flat_map(|m| m.bag().rdf_lis.iter().map(|li| li.resource.as_str()))
                .collect()
        })
    }
}

impl<'a> IntoIterator for &'a Annotation {
    type Item = &'a str;
    type IntoIter = AnnotationIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        AnnotationIterator {
            iter: match &self.rdf {
                Some(rdf) => Box::new(
                    rdf.description
                        .inner
                        .iter()
                        .flat_map(|m| m.bag().rdf_lis.iter().map(|li| li.resource.as_str())),
                ),
                None => Box::new(std::iter::empty()),
            },
        }
    }
}

pub struct AnnotationIterator<'a> {
    iter: Box<dyn Iterator<Item = &'a str> + 'a>,
}

impl<'a> Iterator for AnnotationIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> From<&'a Annotation> for HashMap<&'a str, Vec<&'a str>> {
    fn from(s: &'a Annotation) -> HashMap<&'a str, Vec<&'a str>> {
        s.rdf
            .as_ref()
            .map(|rdf| {
                rdf.description
                    .inner
                    .iter()
                    .flat_map(|m| {
                        m.bag()
                            .rdf_lis
                            .iter()
                            .map(|li| li.resource.split('/').rev().take(2).collect::<Vec<&str>>())
                    })
                    .filter_map(|vec| {
                        if vec.len() == 2 {
                            Some((vec[1], vec[0]))
                        } else {
                            None
                        }
                    })
                    .into_group_map()
            })
            .unwrap_or_default()
    }
}

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
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Default)]
pub struct Rdf {
    #[serde(rename = "$unflatten=rdf:Description", default)]
    pub description: RdfDescriptor,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Default)]
pub struct RdfDescriptor {
    #[serde(rename = "$value", default)]
    pub inner: Vec<Bqbiol>,
}

/// A heavily-hardcoded model qualifier enum for <bqbiol:VARIANT>
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
// }
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
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
}

impl Bqbiol {
    pub fn bag(&self) -> &RdfBag {
        match self {
            Bqbiol::Encodes { rdf_bag: x } => x,
            Bqbiol::HasPart { rdf_bag: x } => x,
            Bqbiol::HasProperty { rdf_bag: x } => x,
            Bqbiol::HasVersion { rdf_bag: x } => x,
            Bqbiol::Is { rdf_bag: x } => x,
            Bqbiol::IsDescribedBy { rdf_bag: x } => x,
            Bqbiol::IsEncodedBy { rdf_bag: x } => x,
            Bqbiol::IsHomologTo { rdf_bag: x } => x,
            Bqbiol::IsPartOf { rdf_bag: x } => x,
            Bqbiol::IsPropertyOf { rdf_bag: x } => x,
            Bqbiol::IsVersionOf { rdf_bag: x } => x,
            Bqbiol::OccursIn { rdf_bag: x } => x,
            Bqbiol::HasTaxon { rdf_bag: x } => x,
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

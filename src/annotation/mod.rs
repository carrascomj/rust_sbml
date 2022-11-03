pub mod rdf;
#[cfg(feature = "sabiork")]
pub mod sabiork;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::From};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
    #[serde(rename = "$unflatten=rdf:RDF")]
    pub rdf: Option<rdf::Rdf>,
    #[cfg(feature = "sabiork")]
    #[serde(rename = "$unflatten=sbrk:sabiork")]
    pub sabiork: Option<sabiork::Sabiork>,
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

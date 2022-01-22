use serde::{Deserialize, Serialize};

/// Annotation for SabioRK. It is usually found under `[base_types::KineticLaws]`
/// of `[base_types::Reaction]`
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
///         <sbrk:sabiork xmlns:sbrk='http://sabiork.h-its.org'>
///             <sbrk:kineticLawID>51860</sbrk:kineticLawID>
///             <sbrk:experimentalConditions>
///                 <sbrk:temperature>
///                     <sbrk:startValueTemperature>25.0</sbrk:startValueTemperature>
///                     <sbrk:temperatureUnit>°C</sbrk:temperatureUnit>
///                 </sbrk:temperature>
///                 <sbrk:pH>
///                     <sbrk:startValuepH>7.4</sbrk:startValuepH>
///                 </sbrk:pH>
///                 <sbrk:buffer>20 mM Tris/HCl</sbrk:buffer>
///             </sbrk:experimentalConditions>
///         </sbrk:sabiork>
///     </annotation>
///     </species>"
/// )
/// .unwrap();
///
/// assert_eq!(
///     species[0].annotation.as_ref().unwrap().sabiork.as_ref().unwrap().kinetic_law_id,
///     51860
/// );
///
/// assert_eq!(species[0].annotation.as_ref().unwrap().sabiork.as_ref().unwrap().get_ph(), Some(7.4));
/// assert_eq!(species[0].annotation.as_ref().unwrap().sabiork.as_ref().unwrap().get_temperature(), Some(25.0));
/// ```
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Default)]
pub struct Sabiork {
    #[serde(rename = "$unflatten=sbrk:kineticLawID", default)]
    pub kinetic_law_id: u32,
    #[serde(rename = "$unflatten=sbrk:experimentalConditions", default)]
    pub experimental_conditions: ExperimentalConditions,
}

impl Sabiork {
    pub fn get_ph(&self) -> Option<f32> {
        self.experimental_conditions
            .ph
            .as_ref()
            .map(|ph| ph.start_value_ph)
    }
    pub fn get_temperature(&self) -> Option<f32> {
        self.experimental_conditions
            .temperature
            .as_ref()
            .map(|t| t.start_value_temperature)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Default)]
pub struct ExperimentalConditions {
    #[serde(rename = "$unflatten=sbrk:pH", default)]
    pub ph: Option<Ph>,
    #[serde(rename = "$unflatten=sbrk:temperature", default)]
    pub temperature: Option<Temperature>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Default)]
pub struct Ph {
    #[serde(rename = "$unflatten=sbrk:startValuepH", default)]
    pub start_value_ph: f32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Default)]
pub struct Temperature {
    #[serde(rename = "$unflatten=sbrk:startValueTemperature", default)]
    pub start_value_temperature: f32,
    #[serde(rename = "$unflatten=sbrk:temperatureUnit", default)]
    pub unit: Option<String>,
}

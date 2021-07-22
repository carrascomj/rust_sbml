use serde::{Deserialize, Serialize, Serializer};

/// Define an enum (harcoded as pub) with a method `name()` to serialize it as
/// a string representing its variant; e.g., A::B.name() == "B".
macro_rules! enum_str {
    ($(#[$outer:meta])*  // capture the docstring
        enum $name:ident {
        $($variant:ident),*,
    }) => {
        #[derive(Debug, Hash, PartialEq, Eq, Deserialize, Serialize, Clone)]
        #[allow(non_camel_case_types)]
        #[serde(rename_all="camelCase")]
        pub enum $name {
            $($variant),*
        }

        impl $name {
            const fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
        }
    };
}

/// Combination of [`Unit`].
///
/// The approach to defining units in SBML is compositional; for example,
/// metre second −2 is constructed by combining
/// an Unit object representing metre with another Unit object representing
/// second −2.
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct UnitDefinition {
    pub id: Option<String>,
    #[serde(rename = "listOfUnits", default)]
    pub list_of_units: ListOfUnits,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Default, Clone)]
pub struct ListOfUnits {
    #[serde(rename = "unit")]
    pub units: Vec<Unit>,
}

/// A Unit object represents a reference to a (possibly transformed) base unit
/// (see [UnitSIdRef]).
///
/// The attribute kind indicates the base unit, whereas the attributes
/// exponent, scale and multiplier define how the base unit is being transformed.
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Unit {
    pub kind: UnitSIdRef,
    pub exponent: f64,
    pub scale: i64,
    pub multiplier: f64,
}

/// SBML provides predefined base units, gathered in [`UnitSId`].
/// Alternatively, one can use arbitrary `CustomUnit`s.
#[derive(Debug, Deserialize, Hash, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum UnitSIdRef {
    #[allow(clippy::upper_case_acronyms)]
    SIUnit(UnitSId),
    CustomUnit(String),
}

impl Serialize for UnitSIdRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::SIUnit(ref unit) => serializer.serialize_str(unit.name()),
            Self::CustomUnit(s) => serializer.serialize_str(s),
        }
    }
}

enum_str! {
/// One of the predefined values of a base unit by SBML level 3.
enum UnitSId {
    ampere,
    avogadro,
    coulomb,
    gray,
    joule,
    litre,
    mole,
    radian,
    steradian,
    weber,
    dimensionless,
    henry,
    katal,
    lumen,
    newton,
    tesla,
    becquerel,
    farad,
    hertz,
    kelvin,
    lux,
    ohm,
    siemens,
    volt,
    candela,
    gram,
    item,
    kilogram,
    metre,
    pascal,
    sievert,
    watt,
    second,
}}

[![Crates.io](https://img.shields.io/crates/v/sbml.svg)](https://crates.io/crates/sbml)
[![Documentation](https://docs.rs/sbml/badge.svg)](https://docs.rs/sbml/)
[![Build](https://github.com/jlricon/sbml/workflows/Build/badge.svg)](https://github.com/jlricon/sbml)
[![Codecov](https://codecov.io/github/jlricon/sbml/coverage.svg?branch=master)](https://codecov.io/gh/jlricon/sbml)

# A parser for SBML
This is a parser for the Systems Biology Markup Language (SBML).

Very early stage! I also wrote a companion MathML parser that this crate uses.

## Example
```rust
let example=r#"<?xml version="1.0" encoding="UTF-8"?>
<sbml xmlns="http://www.sbml.org/sbml/level3/version2/core" level="3" version="2">
    <model timeUnits="second" extentUnits="mole" substanceUnits="mole">
    </model>
</sbml>"#;
let result = parse_document(example);
```
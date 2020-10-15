[![Crates.io](https://img.shields.io/crates/v/rust_sbml.svg)](https://crates.io/crates/rust_sbml)
[![pypi](https://img.shields.io/pypi/v/rust_sbml.svg)](https://pypi.org/project/rust_sbml/)
[![Documentation](https://docs.rs/rust_sbml/badge.svg)](https://docs.rs/rust_sbml/)
[![Build](https://github.com/carrascomj/rust_sbml/workflows/build/badge.svg)](https://github.com/carrascomj/rust_sbml)
[![Codecov](https://codecov.io/github/carrascomj/rust_sbml/coverage.svg?branch=trunk)](https://codecov.io/gh/carrascomj/rust_sbml)

# A parser for SBML
This is a parser for the Systems Biology Markup Language (SBML):
  * [Standalone Rust library](#rust)
  * [Python API](#python)

## Example
```rust
let example=r#"<?xml version="1.0" encoding="UTF-8"?>
<sbml xmlns="http://www.sbml.org/sbml/level3/version2/core" level="3" version="2">
    <model timeUnits="second" extentUnits="mole" substanceUnits="mole">
    </model>
</sbml>"#;
let result = parse_document(example);
```

## Getting started

### Rust
Add it to your Cargo.toml with no default features to avoid all
[PyO3](https://github.com/PyO3/pyo3) nuisances.

```toml
[dependencies.rust_sbml]
version = "0.3.0"
default_features=false
```

### Python
It has only been tested on Linux.
#### Using pip

```shell
pip install rust_sbml
```

#### From source
Clone the repository.
```shell
git clone https://github.com/carrascomj/rust_sbml.git
```
You need [maturin](https://github.com/PyO3/maturin) for building it.
```shell
python -m pip install maturin
```
* Build locally
  ```shell
  maturin build --release
  pip install .
  ```
* Build on virtualenv (no pip install required)
  ```shell
  # --release can be omitted to speed up compilation time
  maturin develop --release
  ```

Having it installed, you can use it as a normal Python package.

```python
from rust_sbml import Model

sbml = Model("examples/EcoliCore.xml")
reaction = sbml.getListOfReactions()[0]
print(reaction.getListOfReactants())
```

### Milestone
* [x] `getListOfSpecies()` (id, name)
* [x] `getListOfCompartments()` (id, name)
* [x] `getListOfReactions()` (id, name)
  * [x] `.getListOfReactants()` (id, name)
  * [x] .`getListOfProducts()` (id, name)
* [x] Capable of retrieving FBC bounds.
* [x] Published to pypi
* [ ] Kinetic Laws.
* [ ] Metadata.
* [x] Test suite with python calls.
* [x] Test suite with libsbml comparison trough cobrapy.

[![Crates.io](https://img.shields.io/crates/v/rust_sbml.svg)](https://crates.io/crates/rust_sbml)
[![pypi](https://img.shields.io/pypi/v/rust_sbml.svg)](https://pypi.org/project/rust_sbml/)
[![Documentation](https://docs.rs/rust_sbml/badge.svg)](https://docs.rs/rust_sbml/)
[![Build](https://github.com/carrascomj/rust_sbml/workflows/build/badge.svg)](https://github.com/carrascomj/rust_sbml)
[![Codecov](https://codecov.io/github/carrascomj/rust_sbml/coverage.svg?branch=trunk)](https://codecov.io/gh/carrascomj/rust_sbml)

# rust_sbml

Parser for the [Systems Biology Markup Language (SBML)](http://sbml.org/Special/specifications/sbml-level-3/version-2/core/release-2/sbml-level-3-version-2-release-2-core.pdf):
  * [Standalone Rust library](#rust)
  * [Python API](#python)

## Getting started

### Rust
Add it to your Cargo.toml with no default features to avoid all
[PyO3](https://github.com/PyO3/pyo3) nuisances.

```toml
[dependencies.rust_sbml]
version = "0.6.0"
default_features=false
```

For example,

```rust
use rust_sbml::Model;

let example=r#"<?xml version="1.0" encoding="UTF-8"?>
<sbml xmlns="http://www.sbml.org/sbml/level3/version2/core" level="3" version="2">
     <model timeUnits="second" extentUnits="mole" substanceUnits="mole">
     </model>
</sbml>"#;
let result = Model::parse(example);
println!("{:?}", result.unwrap());
```

See [write_to_file.rs](https://github.com/carrascomj/rust_sbml/blob/trunk/examples/write_to_file.rs)
for an example on serializing to a file.

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

### Milestones
* `getListOfSpecies()` (id, name)
* `getListOfCompartments()` (id, name)
* `getListOfReactions()` (id, name)
  * `.getListOfReactants()` (id, name)
  * .`getListOfProducts()` (id, name)
* Capable of retrieving FBC bounds.
* Published to pypi
* Kinetic Laws.
* _(Missing) Metadata_.
* Test suite with python calls.
* Test suite with libsbml comparison trough cobrapy.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

> README.md is automatically generated on CI using [cargo-readme](https://github.com/livioribeiro/cargo-readme). Please, modify README.tpl or lib.rs instead (check [the github worflow](https://github.com/carrascomj/rust_sbml/blob/trunk/.github/workflows/readme.yml) for more details).

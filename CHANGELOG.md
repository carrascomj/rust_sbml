Next release
------------
* ...


0.6.1
-----
* Publish crate and upload multi-platform python wheels on CI tags.

0.6.0
-----
* Naive implementation of MathML.
* Add `<math>` attribute to `InitialAssignment`.
* Implement `Rule` and `Function` base types.

0.5.3
-----
* Benchmark against libSBML (through python's [cobrapy](https://github.com/opencobra/cobrapy/))
* Enhance pyo bindings with some removal of superfluous `PyResult`s.
* Add missing SBase attributes to `Species` and `Compartment`.
* Improve test structure and docs.


0.5.2
-----
* Fix serializing of `kind` field in `Unit`.
* Fix on markdown.

0.5.1
-----
* Remove println statement.
* Fixes on markdown.

0.5.0
-----
* Replaced [roxmltree](https://github.com/RazrFalcon/roxmltree) in favor of [quick-xml](https://github.com/tafia/quick-xml/).
* Writing capabilities of `ModelRaw`.
* Huge improvement of documentation.
* Lost support of [MathML](https://github.com/jlricon/mathml/).

0.4.1
-----
* Added in-code README and automate README generation through cargo-readme

0.4.0
-----
* Added model.name and model.id

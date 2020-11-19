# Integration tests
These tests use [pytest](https://docs.pytest.org/en/stable/) to ensure two
objectives (during CI):
1. Test the python interface at [`src/pyo.rs`](https://github.com/carrascomj/rust_sbml/blob/trunk/src/pyo.rs).
2. Ensure the parser is consistent with [libSBML](https://github.com/sbmlteam/libsbml).

The benchmarks are ignored during CI.

## Benchmarks
> The benchmarks are obviously unfair since **rust_sbml** does not cover the entire
[SBML level 3 core specification](https://www.degruyter.com/view/journals/jib/16/2/article-20190021.xml).
Moreover, it does not check for consistency with the extra [plugin FBC](sbml.org/Documents/Specifications/SBML_Level_3/Packages/fbc), and
ignores namespaces (the rest of the packages are not implemented).

Download bigg model (_E. coli_ core is on the repository already):

```shell
curl -L -O http://bigg.ucsd.edu/static/models/RECON1.xml
mv RECON1.xml tests_integration
```

Install required dev dependencies (a compatible version of python is required):

```shell
pip install pytest pytest-benchmark rust_sbml cobra
```

Run the tests from the root of this repository:

```shell
pytest tests_integration
```

Results:
```
----------------------------------------------------------------------------------------------- benchmark: 4 tests -----------------------------------------------------------------------------------------------
Name (time in ms)                         Min                   Max                  Mean              StdDev                Median                 IQR            Outliers      OPS            Rounds  Iterations
------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_benchmark_rust_sbml_small        36.8397 (1.0)         86.1893 (1.0)         39.4792 (1.0)        9.3725 (1.0)         37.5944 (1.0)        0.8622 (1.0)           1;2  25.3298 (1.0)          27           1
test_benchmark_libsbml_small         169.6363 (4.60)       230.0817 (2.67)       180.0127 (4.56)      24.5311 (2.62)       170.0557 (4.52)       0.8629 (1.00)          1;1   5.5552 (0.22)          6           1
test_benchmark_rust_sbml_big       1,570.5220 (42.63)    1,627.4761 (18.88)    1,592.6992 (40.34)     20.9688 (2.24)     1,587.9476 (42.24)     16.3945 (19.01)         2;1   0.6279 (0.02)          5           1
test_benchmark_libsbml_big         5,961.8366 (161.83)   6,595.0936 (76.52)    6,258.9519 (158.54)   236.3963 (25.22)    6,218.8225 (165.42)   310.7440 (360.39)        2;0   0.1598 (0.01)          5           1
------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

```

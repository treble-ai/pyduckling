# PyDuckling
[![Project License - MIT](https://img.shields.io/pypi/l/pyduckling-native.svg)](https://raw.githubusercontent.com/treble-ai/pyduckling-native/master/LICENSE)
[![pypi version](https://img.shields.io/pypi/v/pyduckling-native.svg)](https://pypi.org/project/pyduckling-native/)
[![conda version](https://img.shields.io/conda/vn/conda-forge/pyduckling-native.svg)](https://www.anaconda.com/download/)
[![download count](https://img.shields.io/conda/dn/conda-forge/pyduckling-native.svg)](https://www.anaconda.com/download/)
[![Downloads](https://pepy.tech/badge/pyduckling-native)](https://pepy.tech/project/pyduckling-native)
[![PyPI status](https://img.shields.io/pypi/status/pyduckling-native.svg)](https://github.com/treble-ai/pyduckling-native)
![Linux tests](https://github.com/treble-ai/pyduckling/workflows/Rust/badge.svg)

*Copyright © 2020– Treble.ai*

## Overview
This package provides native bindings for Facebook's [Duckling](https://github.com/facebook/duckling) in Python. This package supports all dimensions and languages available on the original library, and it does not require to spawn a Haskell server and does not use HTTP to call the Duckling API.

## Installing
To install pyduckling, you can use both conda and pip package managers:

```bash
# Using conda (Recommended)
conda install pyduckling

# Using pip (Discouraged)
pip install pyduckling-native
```

**Notes:** Right now, we only provide package distributions for Linux (x86_64). We will provide Windows and Mac distributions on the next release


## Dependencies
To compile pyduckling, you will require the latest nightly release of [Rust](https://rustup.rs/), alongside [Cargo](https://crates.io/). Also, it requires a Python distribution with its corresponding development headers. Finally, this project depends on the following Cargo crates:

* [PyO3](https://github.com/PyO3/pyo3): Library used to produce Python bindings from Rust code.
* [Maturin](https://github.com/PyO3/maturin): Build system to build and publish Rust-based Python packages

Additionally, this package depends on [Duckling-FFI](https://github.com/treble-ai/duckling-ffi), used to compile the native interface to Duckling on Haskell. In order to compile Duckling-FFI, you will require the [Stack](https://haskell-lang.org/get-started) Haskell manager.

Finally, as Duckling-FFI is compiled dynamically, rather than statically, this package requires a working Haskell runtime available.

## Installing locally
Besides Rust and Stack, you will require the latest version of maturin installed to compile this project locally:

```bash
pip install maturin toml
```

First, you will need to compile Duckling-FFI in order to produce the shared library ``libducklingffi``, to do so, you can use the git submodule found at the root of this repository:

```bash
cd duckling-ffi
stack build
```

Then, you will need to move the resulting binary ``libducklingffi.so`` to the ``ext_lib`` folder:

```bash
cp duckling-ffi/libducklingffi.so ext_lib
```

After completing this procedure, it is possible to execute the following command to compile pyduckling:

```bash
maturin develop
```

In order to produce wheels, ``maturin build`` can be used instead. This project supports [PEP517](https://www.python.org/dev/peps/pep-0517/), thus pip can be used to install this package as well:

```bash
pip install -U .
```

## Running tests
We use pytest to run tests as it follows (after calling ``maturin develop``):

```bash
pytest -v duckling/tests
```

## Package usage
PyDuckling provides access to the parsing capabilities of Duckling used to extract structured data from text.

```python
# Core imports
from duckling import (load_time_zones, parse_ref_time,
                      parse_lang, default_locale_lang, parse_locale,
                      parse_dimensions, parse, Context)

# Load reference time for time parsing
time_zones = load_time_zones("/usr/share/zoneinfo")
bog_now = pendulum.now('America/Bogota').replace(microsecond=0)
ref_time = parse_ref_time(
    time_zones, 'America/Bogota', bog_now.int_timestamp)

# Load language/locale information
lang_es = parse_lang('ES')
default_locale = default_locale_lang(lang_es)
locale = parse_locale('ES_CO', default_locale)

# Create parsing context with time and language information
context = Context(ref_time, locale)

# Define dimensions to look-up for
valid_dimensions = ["amount-of-money", "credit-card-number", "distance",
                    "duration", "email", "number", "ordinal",
                    "phone-number", "quantity", "temperature",
                    "time", "time-grain", "url", "volume"]

# Parse dimensions to use
output_dims = parse_dimensions(valid_dimensions)

# Parse a phrase
result = parse('En dos semanas', context, dims, False)
```

This wrapper allows access to all the dimensions and languages available on Duckling:

| Dimension | Example input | Example value output |
| --------- | ------------- | -------------------- |
| `amount-of-money` | "42€" | `{"value":42,"type":"value","unit":"EUR"}` |
| `credit-card-number` | "4111-1111-1111-1111" | `{"value":"4111111111111111","issuer":"visa"}` |
| `distance` | "6 miles" | `{"value":6,"type":"value","unit":"mile"}` |
| `duration` | "3 mins" | `{"value":3,"minute":3,"unit":"minute","normalized":{"value":180,"unit":"second"}}` |
| `email` | "duckling-team@fb.com" | `{"value":"duckling-team@fb.com"}` |
| `number` | "eighty eight" | `{"value":88,"type":"value"}` |
| `ordinal` | "33rd" | `{"value":33,"type":"value"}` |
| `phone-number` | "+1 (650) 123-4567" | `{"value":"(+1) 6501234567"}` |
| `quantity` | "3 cups of sugar" | `{"value":3,"type":"value","product":"sugar","unit":"cup"}` |
| `temperature` | "80F" | `{"value":80,"type":"value","unit":"fahrenheit"}` |
| `time` | "today at 9am" | `{"values":[{"value":"2016-12-14T09:00:00.000-08:00","grain":"hour","type":"value"}],"value":"2016-12-14T09:00:00.000-08:00","grain":"hour","type":"value"}` |
| `url` | "https://api.wit.ai/message?q=hi" | `{"value":"https://api.wit.ai/message?q=hi","domain":"api.wit.ai"}` |
| `volume` | "4 gallons" | `{"value":4,"type":"value","unit":"gallon"}` |


## Changelog
Please see our [CHANGELOG](https://github.com/treble-ai/pyduckling/blob/master/CHANGELOG.md) file to learn more about our new features and improvements.


## Contribution guidelines
We follow PEP8 and PEP257 for pure python packages and Rust to compile extensions. We use MyPy type annotations for all functions and classes declared on this package. Feel free to send a PR or create an issue if you have any problem/question.

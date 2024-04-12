# PDM

_April 11, 2024_

## Goal

Learn how to use pdm to manage dependencies and build python packages.

## Method

1. Install PDM
2. Create a package using `pyproject.toml`
   1. The package will be called `bkmath` and will contain a few math functions.
3. Build the package to a wheel.
4. Create a second (namespace) package that depends on the first.
   1. `bkmath.geometry` adds some basic area and perimeter functions using `bkmath`.
5. "Distribute" the packages in a PEP 503 compliant local folder and see if PDM can import it.

## Status

Seems to all work fine.

To import `bkmath`, execute

```console
pip install bkmath --extra-index-url path
```

and replace `path` with the absolute path of `package_index/`.

<h1 align="center">reuler</h1>
<p align="center">
Solutions to Project Euler in Rust
</p>

<p align="center">
    <a href="https://github.com/astariul/reuler/releases"><img src="https://img.shields.io/github/release/astariul/reuler.svg" alt="GitHub release" /></a>
    <a href="https://github.com/astariul/reuler/actions/workflows/build.yaml"><img src="https://github.com/astariul/reuler/actions/workflows/build.yaml/badge.svg" alt="Build status" /></a>
    <a href="https://github.com/astariul/reuler/blob/main/LICENSE"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="licence" /></a>
</p>

<p align="center">
  <a href="#description">Description</a> •
  <a href="#install">Install</a> •
  <a href="#usage">Usage</a> •
  <a href="#contribute">Contribute</a>
</p>


<h2 align="center">Description</h2>

`reuler` is a crate that contains the solutions for the [Project Euler](https://projecteuler.net/).

Note that this project is a Work In Progress.


<h2 align="center">Install</h2>

Install `reuler` by running :

```
cargo install reuler
```


<h2 align="center">Usage</h2>

You can simply run the command line followed by the ID of the problem your're trying to solve :

```console
reuler <id>
```

---

For example, if you're trying to solve ["Amicable numbers"](https://projecteuler.net/problem=21) (problem #21), just run :

```console
reuler 21
```


<h2 align="center">Contribute</h2>

To contribute, install the package locally, create your own branch, add your code (and tests, and documentation), and open a PR !

### Code formatting

Ensure the code you added is properly formatted with :

```console
cargo fmt
```

### Tests

When you contribute, you need to make sure all the unit-tests pass. You should also add tests if necessary !

You can run the tests with :

```console
cargo test
```

### Documentation

The documentation should be kept up-to-date. You can visualize the documentation locally by running :

```console
cargo doc --open
```
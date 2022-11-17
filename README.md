<h1 align="center">rustere</h1>
<p align="center">
Rust Template repository
</p>

<p align="center">
    <a href="https://github.com/astariul/rustere/releases"><img src="https://img.shields.io/github/release/astariul/rustere.svg" alt="GitHub release" /></a>
    <a href="https://github.com/astariul/rustere/actions/workflows/build.yaml"><img src="https://github.com/astariul/rustere/actions/workflows/build.yaml/badge.svg" alt="Build status" /></a>
    <a href="https://github.com/astariul/rustere/blob/main/LICENSE"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="licence" /></a>
</p>

<p align="center">
  <a href="#description">Description</a> •
  <a href="#install">Install</a> •
  <a href="#usage">Usage</a> •
  <a href="#use-this-template">Use this template</a> •
  <a href="#contribute">Contribute</a>
</p>


<h2 align="center">Description</h2>

**`rustere`** stands for **Rus**t **te**mplate **re**pository.

It's just a template repository for Rust, with the following features :

* :octocat: CI with [Github actions](https://github.com/features/actions) :
  * Code formatting
  * Code checks
  * Unit-tests
* 📝 Issues & PR templates
* 🤖 Stale bot & Dependabot
* 🚀 Releases automatically published to [crates.io](https://crates.io/)


<h2 align="center">Install</h2>

Install `rustere` by running :

```
cargo install rustere
```


<h2 align="center">Usage</h2>

`rustere` does not contain any useful code because it's a template repository.  
But you can run the following command to check if the package was correctly installed :

```console
rustere
```


<h2 align="center">Use this template</h2>

To use this template, click the button "Use this template" :

<p align="center">
  <a href="https://github.com/astariul/rustere/generate"><img src="https://img.shields.io/badge/%20-Use%20this%20template-green?style=for-the-badge&color=347d39" alt="Use template" /></a>
</p>

It will prompt you to create a new Github repository.

Then replace the content in your freshly created repository, with your own package name, own code, and update the links to point to your own repository.

---

Here is an exhaustive list of things to do :

* **Add your content** :
  * **Change `cargo.toml`** : Replace the name of the package, the version, and the description.
  * **Replace `README.md`** : You can keep the same README outline, but you must update the core content to fit what you're building. Make sure to replace any occurence of `astariul` with your own username, and replace any occurence of `rustere` with the name of your package.
  * **Add your code** : Erase the content of `src/main.rs` and `src/lib.rs` and put your own code.
  * **Replace the tests** : Replace the content of `tests/integration_tests.rs` with actual tests.
  * **Update names and links in `.github/` folder** :
    * In `.github/ISSUE_TEMPLATE/bug.yaml`, replace `rustere` with the name of your package.
    * In `.github/ISSUE_TEMPLATE/config.yml`, replace `astariul/rustere` by your own `<user>/<repo>`.
  * Optionally, if there are some features you don't want (like Github action that automatically release your code to `crates.io`), you can remove it !
* **Enable Dependabot** : From the Github website, on your repository page, you can enable [Dependabot](https://docs.github.com/en/code-security/dependabot/dependabot-security-updates/configuring-dependabot-security-updates#enabling-or-disabling-dependabot-security-updates-for-an-individual-repository) by going to the `Settings` tab of your repository, then in the `Security & analysis` section you can enable `Dependabot alerts` and `Dependabot security updates`.
* **Add your `crates.io` API token** : The Github action that automatically publish your package to `crates.io` requires your [API token](https://crates.io/settings/tokens). You can store this API token in a [Github secret](https://docs.github.com/en/actions/security-guides/encrypted-secrets).  
To do this, go to the `Settings` tab of your Github repository, then go to the `Secrets` section, and click the button `New repository secret`.  
Then set the name of the secret as `CARGO_REGISTRY_TOKEN`, and paste your API token in the value field.


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
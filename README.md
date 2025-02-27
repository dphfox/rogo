<div align="center">
    <h1>RoGo</h1>
    <h3>a portable fork of the awesome <a href="https://rojo.space"><img src="assets/brand_images/logo-512.png" alt="Rojo" height="24"/></a> project</h3>
</div>

<div>&nbsp;</div>

<hr />

RoGo is a small fork of Rojo that strictly enforces useful portability features introduced in later Luau versions, but which aren't adopted by the Rojo project. The aim is to foster complete interoperability between all Luau execution environments for large codebases previously built using Rojo, as well as community projects like <a href="https://elttob.uk/go/fusion">Fusion</a>.

Current list of enforced portability features:

- Path stability
	- RoGo completely unimplements promotion of files such as `init.luau` so that every module has a consistent path in all environments.

Learn more about Rojo below!

<hr />

<div align="center">
    <a href="https://github.com/rojo-rbx/rojo/actions"><img src="https://github.com/rojo-rbx/rojo/workflows/CI/badge.svg" alt="Actions status" /></a>
    <a href="https://crates.io/crates/rojo"><img src="https://img.shields.io/crates/v/rojo.svg?label=latest%20release" alt="Latest server version" /></a>
    <a href="https://rojo.space/docs"><img src="https://img.shields.io/badge/docs-website-brightgreen.svg" alt="Rojo Documentation" /></a>
</div>

<hr />

**Rojo** is a tool designed to enable Roblox developers to use professional-grade software engineering tools.

With Rojo, it's possible to use industry-leading tools like **Visual Studio Code** and **Git**.

Rojo is designed for power users who want to use the best tools available for building games, libraries, and plugins.

## Features
Rojo enables:

* Working on scripts and models from the filesystem, in your favorite editor
* Versioning your game, library, or plugin using Git or another VCS
* Streaming `rbxmx` and `rbxm` models into your game in real time
* Packaging and deploying your project to Roblox.com from the command line

In the future, Rojo will be able to:

* Sync instances from Roblox Studio to the filesystem
* Automatically convert your existing game to work with Rojo
* Import custom instances like MoonScript code

## [Documentation](https://rojo.space/docs)
Documentation is hosted in the [rojo.space repository](https://github.com/rojo-rbx/rojo.space).

## Contributing
Check out our [contribution guide](CONTRIBUTING.md) for detailed instructions for helping work on Rojo!

Pull requests are welcome!

Rojo supports Rust 1.70.0 and newer. The minimum supported version of Rust is based on the latest versions of the dependencies that Rojo has.

## License
Rojo is available under the terms of the Mozilla Public License, Version 2.0. See [LICENSE.txt](LICENSE.txt) for details.

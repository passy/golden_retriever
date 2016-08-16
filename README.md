[![Build Status](https://travis-ci.org/woofwoofinc/csearch.svg?branch=master)](https://travis-ci.org/woofwoofinc/csearch)
[![Dependency Status](https://dependencyci.com/github/woofwoofinc/csearch/badge)](https://dependencyci.com/github/woofwoofinc/csearch)
[![Windows Build Status](https://ci.appveyor.com/api/projects/status/nve0g5owu810pi4w/branch/master?svg=true)](https://ci.appveyor.com/project/passy/csearch)
[![License](https://img.shields.io/github/license/woofwoofinc/csearch.svg)](https://github.com/woofwoofinc/csearch/blob/master/LICENSE)


CSearch
=======
Making Code Search Great Again.


Developing CSearch
------------------
Install the [Rust] development tools on your system with [rustup] if they are
not already available. Then build and test the project using:

    cargo test

[Rust]: https://www.rust-lang.org
[rustup]: https://www.rustup.rs


Docker
------
A [Docker] container definition is provided with installations of the tools
used to develop CSearch. To use the container, first install Docker if not
already available and start a Docker terminal. Then create the container by
running the following build at the top level of the repository source tree:

    docker build --rm=true -t csearch .

[Docker]: http://docker.io

Once built, an interactive shell can be run in the container using:

    docker run -it -v "$(pwd):/csearch" --workdir=/csearch csearch /bin/bash

The current working directory from the host machine is available as the current
directory in the container so it is possible to build and test the library as
described earlier.

    cargo test


Running Clippy Lints
--------------------
[Clippy] is a Rust linter. Currently it has to be run manually since CSearch
targets Rust stable and Clippy requires Rust nightly. Switching versions is
easy with `rustup` - use the following to lint the repository:

    rustup run nightly cargo clippy

[Clippy]: https://github.com/Manishearth/rust-clippy


License
-------

    Copyright [2016] [Woof Woof, Inc.]

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.

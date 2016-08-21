[![Build Status](https://travis-ci.org/woofwoofinc/golden_retriever.svg?branch=master)](https://travis-ci.org/woofwoofinc/golden_retriever)
[![Dependency Status](https://dependencyci.com/github/woofwoofinc/golden_retriever/badge)](https://dependencyci.com/github/woofwoofinc/golden_retriever)
[![Windows Build Status](https://ci.appveyor.com/api/projects/status/nve0g5owu810pi4w/branch/master?svg=true)](https://ci.appveyor.com/project/passy/csearch)
[![License](https://img.shields.io/github/license/woofwoofinc/golden_retriever.svg)](https://github.com/woofwoofinc/golden_retriever#license)


Golden Retriever
================
Yes we can... code search.


Developing Golden Retriever
---------------------------
Install the [Rust] development tools on your system with [rustup] if they are
not already available. Then build and test the project using:

    cargo test

[Rust]: https://www.rust-lang.org
[rustup]: https://www.rustup.rs


Docker
------
A [Docker] container definition is provided with installations of the tools
used to develop Golden Retriever. To use the container, first install Docker if
not already available and start a Docker terminal. Then create the container by
running the following build at the top level of the repository source tree:

    docker build --rm=true -t grrr .

[Docker]: http://docker.io

Once built, an interactive shell can be run in the container using:

    docker run -it -v "$(pwd):/grrr" --workdir=/grrr grrr /bin/bash

The current working directory from the host machine is available as the current
directory in the container so it is possible to build and test the library as
described earlier.

    cargo test


Travis
------
Golden Retriever is continuously integrated on [Travis CI].

To update encrypted credentials in the `.travis.yml` file, use the
[Travis command line tool].

    gem install travis

For instance, to update the Slack notification credential:

    travis encrypt "woofwoofinc:<credential>" --add notifications.slack

[Travis CI]: https://travis-ci.org
[Travis command line tool]: https://docs.travis-ci.com/user/encryption-keys


Running Clippy Lints
--------------------
[Clippy] is a Rust linter. Currently it has to be run manually since Golden
Retriever targets Rust stable and Clippy requires Rust nightly. Switching
versions is easy with `rustup` - use the following to lint the repository:

    rustup run nightly cargo clippy

[Clippy]: https://github.com/Manishearth/rust-clippy


License
-------
This work is dual-licensed under the Apache License, Version 2.0 and under the
MIT Licence.

You may licence this work under the Apache License, Version 2.0.

    Copyright 2016 Woof Woof, Inc. and contributors

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.

Alternatively, you may licence this work under the MIT Licence at your option.

    Copyright (c) 2016 Woof Woof, Inc. and contributors
    
    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:
    
    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.
    
    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.

The licence explainers at [Choose a License] may be helpful. They have 
descriptions for both the [Apache 2.0 Licence] and [MIT Licence] conditions.

[Choose a License]: http://choosealicense.com
[Apache 2.0 Licence]: http://choosealicense.com/licenses/apache-2.0/
[MIT Licence]: http://choosealicense.com/licenses/mit/


Contribution
------------
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

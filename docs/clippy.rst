Clippy
------
Clippy_ is a Rust linter. Currently it has to be run manually since Golden
Retriever targets Rust stable and Clippy requires Rust nightly. Switching
versions is easy with ``rustup`` - use the following to lint the repository:

.. code:: bash

    rustup run nightly cargo clippy

.. _Clippy: https://github.com/Manishearth/rust-clippy

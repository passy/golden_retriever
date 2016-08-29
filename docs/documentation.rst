Documentation
-------------
The project documentation under ``docs`` can be compiled using Cargo Sphinx.
Output is placed in ``docs/_build/html``.

The following dependencies are needed to build Cargo Sphinx.

* CMake_: Needed by Cargo tool library dependency.
* Libssl_: Needed by Cargo tool library dependency.

.. _CMake: https://cmake.org
.. _Libssl: https://wiki.openssl.org/index.php/Libssl_API

Install Cargo Sphinx if it is not available.

.. code:: bash

    cargo install cargo-sphinx

Then generate the documentation.

.. code:: bash

    cargo sphinx

The Docker container provides an installation of Python and Sphinx which can be
used to build the documentation also. To make the documentation directly in
container without an intermediate shell, use:

.. code:: bash

    docker run -v "$(pwd):/grrr" \
         --workdir=/grrr \
         grrr \
         cargo sphinx

The compiled document is written to the shared location and is available on the
host machine under ``docs/_build/html``.

It is published to `woofwoofinc.github.io/golden_retriever`_ using
`GitHub Pages`_.

.. _woofwoofinc.github.io/golden_retriever: https://woofwoofinc.github.io/golden_retriever
.. _GitHub Pages: https://pages.github.com

Publishing from the Docker container fails for missing GitHub credentials. In
this case it is possible to run the publication command in the container
interactively and complete it on the host machine. Compile and generate the
Git repository to push in ``docs/_build/html`` by running the following in the
container.

.. code:: bash

    cargo sphinx --push

Then on the host, change to ``docs/_build/html` which is now a new Git
repository with the documentation HTML committed on ``master``. Push this to
``origin`` by specifying the remote.

.. code:: bash

    cd docs/_build/html
    git push -f git@github.com:woofwoofinc/golden_retriever.git master:gh-pages

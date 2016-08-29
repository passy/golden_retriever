Docker
------
A Docker_ container definition is provided with installations of the tools used
to develop Golden Retriever. To use the container, first install Docker if not
already available. Then create the container by running the following build at
the top level of the repository source tree:

.. code:: bash

    docker build -t grrr .

.. _Docker: http://docker.io

Docker uses a build cache so it may be necessary to add ``--no-cache=true``
sometimes in order to achieve a complete rebuild.

Once built, an interactive shell can be run in the container using:

.. code:: bash

    docker run -it \
         -v "$(pwd):/grrr" \
         --workdir=/grrr \
         grrr \
         /bin/bash

The current working directory from the host machine, i.e. the Golden Retriever
repository, is available as the current directory in the container. So it is
possible to build and test the library as described earlier.

.. code:: bash

    cargo test

To tear down the container and start over, remove the Docker image:

.. code:: bash

    docker rmi -f grrr

Docker images can be listed using:

.. code:: bash

    docker images 


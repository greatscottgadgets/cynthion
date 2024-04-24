================================================
Getting Started with Cynthion
================================================


Prerequisites
-------------

To use Cynthion you will need to ensure the following software is installed:

 * `Python <https://wiki.python.org/moin/BeginnersGuide/Download>`__ v3.8, or later.
 * `Rust <https://doc.rust-lang.org/book/ch01-01-installation.html>`__ v1.TODO, or later.


Cynthion Host Software Installation
-----------------------------------

..  TODO uncomment once there is at least one distribution with packages

    The recommended way of installing the Cynthion host software is to use your operating system's package manager but you can also install it from the `Python Package Index <https://pypi.org/project/cynthion/>`__ or directly from `source <https://github.com/greatscottgadgets/cynthion/>`__.


    Install Using Package Managers
    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

    TODO os-specific package manager command

You can install the Cynthion host software from the `Python Package Index (PyPI) <https://pypi.org/project/cynthion/>`__ or directly from `source <https://github.com/greatscottgadgets/cynthion/>`__.


Install From PyPI and crates.io
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Until packages are available for your operating system's package manager we recommend installing the Cynthion host software from PyPI.

For example, you can use the `pip <https://pypi.org/project/pip/>`__ tool to install the Cynthion host software using:

.. code-block :: sh

    pip install cynthion

For more information on installing Python packages from PyPI please refer to the `"Installing Packages" <https://packaging.python.org/en/latest/tutorials/installing-packages/>`__ section of the Python Packaging User Guide.


Install From Source
^^^^^^^^^^^^^^^^^^^

Clone the repository:

.. code-block :: sh

    git clone https://github.com/greatscottgadgets/cynthion.git
    cd cynthion/

Install the cynthion host package:

.. code-block :: sh

    cd cynthion/python/
    pip install .



Test Installation
-----------------

Connect Hardware
^^^^^^^^^^^^^^^^

.. image:: ../images/cynthion_connections.png
  :alt: TODO Cynthion hardware connections to host, target and device


Test hardware connectivity
^^^^^^^^^^^^^^^^^^^^^^^^^^

Open a terminal and run:

.. code-block :: sh

    cynthion info

You should see output like the following:

.. code-block :: sh

    TODO cynthion info output when running as default gateware

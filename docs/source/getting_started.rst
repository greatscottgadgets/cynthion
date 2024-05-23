================================================
Getting Started with Cynthion
================================================


Prerequisites
-------------

To use Cynthion you will need to ensure the following software is installed:

 * `Python <https://wiki.python.org/moin/BeginnersGuide/Download>`__ v3.8, or later.


Cynthion Host Software Installation
-----------------------------------

You can install the Cynthion host software from the `Python Package Index (PyPI) <https://pypi.org/project/cynthion/>`__ or :doc:`directly from source <developer/introduction>`.

To install the GreatFET host software from PyPI using the `pip <https://pypi.org/project/pip/>`__ tool:

.. code-block :: sh

    pip install cynthion

.. note::

    For more information on installing Python packages from PyPI please refer to the
    `"Installing Packages" <https://packaging.python.org/en/latest/tutorials/installing-packages/>`__
    section of the Python Packaging User Guide.


Install udev Rules (Linux Only)
-------------------------------

Configure your system to allow access to Cynthion for logged in users:

.. code-block :: sh

    # install udev rules
    sudo cp util/54-cynthion.rules /etc/udev/rules.d

    # reload udev rules
    sudo udevadm control --reload

    # apply udev rules to any devices that are already plugged in
    sudo udevadm trigger


Test Installation
-----------------

Connect Hardware
^^^^^^^^^^^^^^^^

.. image:: ../images/cynthion-connections-host.svg
  :width: 800
  :alt: Connection diagram for testing Cynthion installation.

- Connect the Host computer to the Cynthion Control port.
- Check that the :ref:`LED A power-on indicator <introduction:Top View>` lights up.
- Check that the :ref:`LED E fault indicator <introduction:Top View>` remains off.


Test Hardware Connectivity
^^^^^^^^^^^^^^^^^^^^^^^^^^

Open a terminal and confirm that everything is working by running:

.. code-block :: sh

    cynthion info

If everything is working you will see the following output:

.. code-block :: text

    Detected a Cynthion device!
        Hardware: Cynthion r1.4
        Serial number: <snip>

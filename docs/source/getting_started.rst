=============================
Getting Started with Cynthion
=============================


Prerequisites
-------------

To use Cynthion you will need to ensure the following software is installed:

 * `Python <https://wiki.python.org/moin/BeginnersGuide/Download>`__ v3.8, or later.


Cynthion Host Software Installation
-----------------------------------

The Cynthion host software distribution can be obtained from the `Python Package Index (PyPI) <https://pypi.org/project/cynthion/>`__ or :doc:`directly from source <developer/introduction>`.

Using the `pip <https://pypi.org/project/pip/>`__ tool you can install the Cynthion host software by running:

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

    sudo cynthion setup

If you'd prefer to perform this step manually, you can download and install the rules as follows:

.. code-block :: sh

    # download udev rules
    curl -O https://raw.githubusercontent.com/greatscottgadgets/cynthion/main/cynthion/python/assets/54-cynthion.rules

    # install udev rules
    sudo cp 54-cynthion.rules /etc/udev/rules.d

    # reload udev rules
    sudo udevadm control --reload

    # apply udev rules to any devices that are already plugged in
    sudo udevadm trigger

You can check if the rules are installed correctly with:

.. code-block :: sh

    cynthion setup --check


Test Installation
-----------------

Connect Hardware
^^^^^^^^^^^^^^^^

.. image:: ../images/cynthion-connections-host.svg
  :width: 800
  :alt: Connection diagram for testing Cynthion installation.

- Connect the Host computer to the Cynthion **CONTROL** port.
- Check that the :ref:`LED A power-on indicator <introduction:Top View>` lights up.


Test Hardware Connectivity
^^^^^^^^^^^^^^^^^^^^^^^^^^

Open a terminal and confirm that everything is working by running:

.. code-block :: sh

    cynthion info --force-offline

If everything is working you will see the following output:

.. code-block :: text

    Detected a Cynthion device!
        Hardware: Cynthion r1.4
        Serial number: xxxxxxxxxxxxxxxxxxxxxxxxxx
        Firmware version: v1.0.4
        USB API version: 1.1
        Flash UID: xxxxxxxxxxxxxxxx


Updating Cynthion Host Software
-------------------------------

To update the Cynthion host software to the latest version run:

.. code-block :: sh

    pip install --upgrade cynthion


Updating Cynthion Microcontroller Firmware and FPGA configuration flash
-----------------------------------------------------------------------

To upgrade the Cynthion Microcontroller firmware and FPGA configuration flash to the latest versions run:

.. code-block :: sh

    cynthion update

You can update the Microcontroller firmware separately with:

.. code-block :: sh

    cynthion update --mcu-firmware

You can update the FPGA configuration flash separately with:

.. code-block :: sh

    cynthion update --bitstream

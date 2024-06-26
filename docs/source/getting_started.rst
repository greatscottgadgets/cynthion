================================================
Getting Started with Cynthion
================================================


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

    # download udev rules
    curl -O https://raw.githubusercontent.com/greatscottgadgets/cynthion/main/util/54-cynthion.rules

    # install udev rules
    sudo cp 54-cynthion.rules /etc/udev/rules.d

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



Upgrading Cynthion Host Software
--------------------------------

To upgrade the Cynthion host software to the latest version run:

.. code-block :: sh

    pip install --upgrade cynthion


Upgrading Cynthion Device Firmware
----------------------------------

To upgrade the Cynthion device firmware to the latest version run:

.. code-block :: sh

    cynthion mcu-firmware --autoflash

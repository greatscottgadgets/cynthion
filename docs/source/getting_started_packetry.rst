============================
Using Cynthion with Packetry
============================

Before proceeding, please ensure you have completed all steps in the :doc:`getting_started` section.

Prerequisites
-------------

To use Cynthion's USB Analyzer you will need to ensure the following software is installed:

* `Packetry <https://github.com/greatscottgadgets/packetry/>`__


USB Analyzer Bitstream
----------------------

Cynthion ships from the factory with the USB Analyzer as the default bitstream for the FPGA.

If you have previously flashed a different default bitstream you can run the USB Analyzer bitstream with:

.. code-block :: sh

    cynthion run analyzer

If you want to configure USB Analyzer as the default bitstream for the FPGA:

.. code-block :: sh

    cynthion flash analyzer

You can verify that everything is working by running:

.. code-block :: sh

    cynthion info

You should see output like:

.. code-block :: sh

    Detected a Cynthion device!
        Bitstream: USB Analyzer (Cynthion Project)
        Hardware: Cynthion r1.4
        Flash UID: xxxxxxxxxxxxxxxx


Connect Hardware
----------------

.. image:: ../images/cynthion-connections-packetry.svg
  :width: 800
  :alt: Connection diagram for using Cynthion with Packetry.

Simple USB Gateware Device
==========================

This tutorial walks through the process of implementing a USB device with Cynthion and `LUNA <https://luna.readthedocs.io/>`__. We'll implement a simple device with two bulk endpoints. The goal of this tutorial is to demonstrate how to send and receive data between a host and a Cynthion gateware device.

Prerequisites
-------------

 * Install the Cynthion tools by following :doc:`/getting_started`.
 * Complete the :doc:`/tutorials/gateware_blinky` tutorial.


How do USB Bulk Transfers Work?
-------------------------------



Device Descriptor
-----------------



Device Endpoints
----------------

* Map inputs & outputs to endpoints.
* IN endpoint is the state of the Cynthion USER button.
* OUT endpoint is the Cynthion FPGA LEDs.



Implement the Bulk Out Endpoint
-------------------------------




Implement the Bulk In Endpoint
------------------------------


Testing the Device
------------------



Exercises
---------

1. Modify the tutorial to work with a Cynthion USER PMOD port
2.

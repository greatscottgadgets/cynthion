SHELL := /bin/zsh

UART	?= /dev/ttyACM0

# - gateware ------------------------------------------------------------------

analyzer:
	python3 -m cynthion.gateware.analyzer.top

soc:
	# generate svd description of design
	mkdir -p build/gensvd/
	python3 -m cynthion.gateware.soc.top --generate-svd > build/gensvd/lunasoc.svd

	# generate rust linker map for design
	mkdir -p build/genrust/
	python3 -m cynthion.gateware.soc.top --generate-memory-x > build/genrust/memory.x
	cp build/genrust/memory.x ../../firmware/memory.x

	# generate bitstream for design
	python3 -m cynthion.gateware.soc.top --output build/$@.bit


# - helpers -------------------------------------------------------------------

clean:
	-rm -rf build/

console:
	pyserial-miniterm $(UART) 115200

tests:
	python3 -m unittest

.PHONY: tests

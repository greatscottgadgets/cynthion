SHELL := /bin/zsh

UART	?= /dev/ttyACM0

# - gateware ------------------------------------------------------------------

analyzer:
	rm -rf build
	python3 -m cynthion.gateware.analyzer.top

soc:
	rm -rf build
	mkdir -p build/gensvd/
	mkdir -p build/genrust/

	# generate svd description of design
	python3 -m cynthion.gateware.soc.top --generate-svd > build/gensvd/lunasoc.svd

	# generate rust linker map for design
	python3 -m cynthion.gateware.soc.top --generate-memory-x > build/genrust/memory.x
	cp build/genrust/memory.x ../../firmware/memory.x

	# generate bitstream for design
	python3 -m cynthion.gateware.soc.top --keep-files


# - helpers -------------------------------------------------------------------

clean:
	-rm -rf build/

console:
	pyserial-miniterm $(UART) 115200

load:
	cynthion configure build/top.bit

tests:
	python3 -m unittest

.PHONY: tests

SHELL := /bin/zsh

UART	?= /dev/ttyACM0

# - gateware ------------------------------------------------------------------

analyzer:
	python3 -m cynthion.gateware.analyzer.top

soc:
	mkdir -p build/
	python3 -m cynthion.gateware.soc.top --output build/$@.bit

svd:
	mkdir -p build/gensvd/
	python3 -m cynthion.gateware.soc.top --generate-svd > build/gensvd/lunasoc.svd


# - helpers -------------------------------------------------------------------

clean:
	-rm -rf build/

console:
	pyserial-miniterm $(UART) 115200

tests:
	python3 -m unittest

.PHONY: tests

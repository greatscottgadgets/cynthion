SHELL := /bin/zsh

UART	?= /dev/ttyACM0

# - gateware ------------------------------------------------------------------

analyzer:
	rm -rf build
	python3 -m cynthion.gateware.analyzer.top

soc:
	rm -rf build
	python3 -m cynthion.gateware.soc.top
	cp build/genrust/memory.x ../../firmware/memory.x


# - helpers -------------------------------------------------------------------

clean:
	-rm -rf build/

console:
	pyserial-miniterm $(UART) 115200

load:
	apollo configure build/top.bit

tests:
	python3 -m unittest

.PHONY: tests

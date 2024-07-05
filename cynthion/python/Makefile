SHELL := /bin/zsh

UART	?= /dev/ttyACM0

# - gateware ------------------------------------------------------------------

analyzer:
	python3 -m cynthion.gateware.analyzer.top --output build/$@.bit

facedancer:
	mkdir -p build/
	python3 -m cynthion.gateware.facedancer.top --output build/$@.bit

svd:
	mkdir -p build/gensvd/
	python3 -m cynthion.gateware.facedancer.top --generate-svd > build/gensvd/lunasoc.svd


# - helpers -------------------------------------------------------------------

clean:
	-rm -rf build/

console:
	pyserial-miniterm $(UART) 115200

load:
	cynthion configure build/facedancer.bit

tests:
	python3 -m unittest

.PHONY: tests


# - packaging -----------------------------------------------------------------

LUNA_PLATFORMS = $(shell python -c \
	"import inspect, cynthion.gateware.platform; \
	 platforms = [name for name, obj in inspect.getmembers(cynthion.gateware.platform) \
		 if inspect.isclass(obj)] ; \
	 print('\n'.join(platforms))" \
)

binaries: assets/moondancer.bin
bitstreams: analyzer.bit selftest.bit facedancer.bit

assets/moondancer.bin:
	cd ../../firmware/moondancer && cargo objcopy --release -- -Obinary ../../cynthion/python/assets/moondancer.bin
	# TODO append firmware to bitstream @ 0x0b0000
	#dd if=assets/moondancer.bin of=assets/facedancer.bit obs=1 seek=720896

%.bit:
	@echo "building" $< -o $@
	@for lp in $(LUNA_PLATFORMS); do \
		if [ -s "assets/$$lp/$@" ]; then \
			echo "Skipping: $$lp/$@"; \
		else; \
			mkdir -p assets/$$lp/; \
			echo "Building: assets/$$lp/$@"; \
			LUNA_PLATFORM="cynthion.gateware.platform:$$lp" \
			python -m cynthion.gateware.$(basename $@).top --dry-run --output "assets/$$lp/$@"; \
		fi; \
	done

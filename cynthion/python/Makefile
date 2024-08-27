SHELL := /bin/zsh

UART	?= /dev/ttyACM0

# - gateware ------------------------------------------------------------------

analyzer:
	python3 -m cynthion.gateware.analyzer.top --output build/$@.bit

selftest:
	mkdir -p build/
	python3 -m cynthion.gateware.selftest.top --output build/$@.bit

facedancer:
	mkdir -p build/
	python3 -m cynthion.gateware.facedancer.top --output build/$@.bit

svd:
	mkdir -p build/gensvd/
	python3 -m cynthion.gateware.facedancer.top --generate-svd > build/gensvd/lunasoc.svd


# - helpers -------------------------------------------------------------------

clean:
	-rm -rf build/

distclean: clean
	-rm -rf assets/CynthionPlatformRev* assets/moondancer.bin

console:
	pyserial-miniterm $(UART) 115200

tests:
	python3 -m unittest

.PHONY: tests


# - development ---------------------------------------------------------------

assets: assets-bitstreams assets/moondancer.bin

assets-bitstreams: detect-fpga-toolchain detect-luna-platform
	@mkdir -p 'assets/$(PLATFORM_NAME)'
	@if [ ! -s "assets/$(PLATFORM_NAME)/analyzer.bit" ]; then \
		python -m cynthion.gateware.analyzer.top --output 'assets/$(PLATFORM_NAME)/analyzer.bit' ; \
	fi
	@if [ ! -s "assets/$(PLATFORM_NAME)/facedancer.bit" ]; then \
		python -m cynthion.gateware.facedancer.top --output 'assets/$(PLATFORM_NAME)/facedancer.bit' ; \
	fi
	@if [ ! -s "assets/$(PLATFORM_NAME)/selftest.bit" ]; then \
		python -m cynthion.gateware.selftest.top --output 'assets/$(PLATFORM_NAME)/selftest.bit' ; \
	fi

assets/moondancer.bin: detect-rust-toolchain
	cd ../../firmware/moondancer && \
	cargo objcopy --release -- -Obinary ../../cynthion/python/assets/moondancer.bin


# - packaging -----------------------------------------------------------------

LUNA_PLATFORMS = $(shell python -c \
	"import inspect, cynthion.gateware.platform; \
	 platforms = [name for name, obj in inspect.getmembers(cynthion.gateware.platform) \
		 if inspect.isclass(obj)] ; \
	 print('\n'.join(platforms))" \
)

binaries:	detect-rust-toolchain assets/moondancer.bin
bitstreams: detect-fpga-toolchain analyzer.bit selftest.bit facedancer.bit

%.bit: detect-fpga-toolchain
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


# - environment checks --------------------------------------------------------

detect-fpga-toolchain:
	@which nextpnr-ecp5 > /dev/null 2>&1 || [ $$? -eq 0 ] \
		|| ( echo "No FPGA toolchain detected. Please install: https://github.com/YosysHQ/oss-cad-suite-build/" ; exit 1)
	@echo "Using FPGA toolchain: $(shell dirname $(shell which nextpnr-ecp5))" \

detect-rust-toolchain:
	@which rustup > /dev/null 2>&1 || [ $$? -eq 0 ] \
		|| ( echo "No Rust toolchain detected. Please install https://rustup.rs/" ; exit 1)
	@rustup component list --installed | grep rust-std-riscv32imac-unknown-none-elf > /dev/null 2>&1 || [ $$? -eq 0 ] \
		|| ( echo "No llvm tools detected. Please run: 'rustup component add llvm-tools'" ; exit 1)
	@which cargo-objcopy > /dev/null 2>&1 || [ $$? -eq 0 ] \
		|| ( echo "No cargo-binutils detected. Please run: 'cargo install cargo-binutils'" ; exit 1)
	@echo "Using Rust toolchain: $(shell rustup show home) $(shell rustup show active-toolchain)"

detect-luna-platform: detect-cynthion-cli
ifeq ($(origin LUNA_PLATFORM), undefined)
	@echo "LUNA_PLATFORM is not set, trying to autodetect it."

	@cynthion info > /dev/null 2>&1 || [ $$? -eq 0 ] \
		|| ( echo "No Cynthion device detected." ; \
			 echo "" ; \
			 echo "Please ensure Cynthion is plugged in or set the LUNA_PLATFORM variable, for" ; \
			 echo "example:" ; \
			 echo "" ; \
			 echo " LUNA_PLATFORM=cynthion.gateware.platform:CynthionPlatformRev1D4 make assets" ; \
			 echo "" ; \
			 exit 1)

	@cynthion build --luna-platform	> /dev/null 2>&1 || [ $$? -eq 0 ] \
		|| ( echo "Unsupported cynthion command line tool detected. Please upgrade to the latest version." ; exit 1)

	$(eval LUNA_PLATFORM := $(shell cynthion build --luna-platform 2> /tmp/cynthion-assets.log))
	@if [ ! -n "$(LUNA_PLATFORM)" ]; then \
		echo "Failed detect luna platform." ; \
		echo "" ; \
		echo "Please see '/tmp/cynthion-assets.log' for more information." ; \
		echo "" ; \
		exit 1 ; \
	fi
endif
	$(eval PLATFORM_NAME := $(shell echo $(LUNA_PLATFORM) | cut -d':' -f2))
	@echo Using LUNA_PLATFORM: $(LUNA_PLATFORM)

detect-cynthion-cli:
	@which cynthion > /dev/null 2>&1 || [ $$? -eq 0 ] \
		|| ( echo "No cynthion command line tool detected. Please run: 'pip install -e .'" ; exit 1)

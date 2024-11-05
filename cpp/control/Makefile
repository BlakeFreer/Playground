BUILD := build

.PHONY: build config clean

build: config
	cmake --build build

config:
	mkdir -p $(BUILD)
	cmake -B$(BUILD) -S . -G"Unix Makefiles"

clean:
	rm -rf $(BUILD)
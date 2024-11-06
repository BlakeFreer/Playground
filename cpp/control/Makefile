BUILD := build

.PHONY: all config clean

all: config
	cmake --build build

examples/%.cpp: config
	cmake --build build --target $*

config:
	mkdir -p $(BUILD)
	cmake -B$(BUILD) -S . -G"Unix Makefiles" -DCMAKE_BUILD_TYPE=Debug

clean:
	rm -rf $(BUILD)
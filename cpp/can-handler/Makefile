BUILD = build

.PHONY: build config clean
build: config
	cmake --build $(BUILD)

config:
	cmake -B $(BUILD) -S. -G"Unix Makefiles"

clean:
	rm -rf $(BUILD)
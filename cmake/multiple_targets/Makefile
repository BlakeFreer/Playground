# This makefile simplifies the commands outlined in the README

# without the --target flag, cmake builds all targets
all: config
	cmake --build build
	./build/adder.exe 4 7
	./build/subber.exe 10 3

# only build "adder"
add-only: config
	cmake --build build --target adder
	./build/adder.exe 4 7

# only build "subber"
sub-only: config
	cmake --build build --target subber
	./build/subber.exe 10 3
		
config:
	cmake -S . -Bbuild -G "Unix Makefiles"

clean:
	rm build -r
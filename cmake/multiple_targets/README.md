# CMake: Multiple Targets

_March 19, 2024_

To test the McMaster Formula [racecar/firmware code](https://github.com/macformula/racecar/tree/main/firmware), I would like to be able to specify a separate test target that can be built independent of the actual ECU projects.

In this sub-project, I investigate building multiple targets with CMake. The testing aspect is not performed here.

## Description

Two console applications are provided, `adder.exe` and `subber.exe`, take 2 command line integers and perform the corresponding operation on them.

Both programs use the `mymath` library which provides some trivial functions.

## Compiling

To build the programs, first configure CMake with

```bash
cmake -S . -B build -G "Unix Makefiles"
```

Then build with one of the following commands

```bash
cmake --build build
cmake --build build --target adder
cmake --build build --target subber
```

When no target is specified (as in the first build command), CMake will build all targets.

Depending on the build command, CMake will place executables `adder.exe` and/or `subber.exe` in the `build/` folder. Run either script to see the basic output.

```bash
./build/adder.exe 5 8
Adding 5 + 8 = 13

./build/subber.exe 12 9
Subtracting 12 - 9 = 3
```

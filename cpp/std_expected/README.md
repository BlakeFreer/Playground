# `std::expected`

Test out how errors might be used with this C++23 feature.

## `main.cc`

```bash
make main
./main 5 8
./main 5 0
```

CLI dividing calculator. Uses `std::expected` to parse the CLI args and `std::optional` to handle divide by zero.

The CLI `parse()` method returns `std::expected<Config, parse_error>` where `Config` is a struct holding valid arguments and `parse_error` is an enum of possible errors. The error enum is passed to a `switch` statement to handle errors differently depending on their type.

This error handling method is rough equivalent to

```c++
Config parse(int argc, char* argv[]) {
    if(argc != 3) throw ParseErrorWrongNumber();
    float a, b;
    try {
        a = std::stof(argv[1]);
        b = std::stof(argv[2]),
    } catch (const std::invalid_argument& e) {
        throw ParseErrorInvalidArgument();
    }
    return Config{a, b};
}

int main(int argc, char* argv[]){
    try {
        Config config = parse(...);
    } catch (const ParseErrorWrongNumber& e) {
        // handle wrong number of args
    } catch (const ParseErrorInvalidFloat& e){
        // handle invalid arg
    }
}
```

## `rusty.cc`

```bash
make rusty
./rusty
```

The `rusty.h` header file defines typedefs and functions to make C++ exceptions and optionals handle like Rust's Result and Option. I wrote `match()` statements for both types which resemble Rust's `match`, though imperfectly.

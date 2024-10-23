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
    try {
        Config {
            std::stof(argv[1]),
            std::stof(argv[2]),
        }
    } catch (const std::invalid_argument& e) {
        throw ParseErrorInvalidArgument();
    }
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

In `parse()`, I had to wrap most of the logic in a `try-catch` block since `std::stof` can _throw_ an exception. This block would not be necessary the function returned `std::expected`, though we would need different ways to halt execution since C++ does not have anything akin to Rust's `?` operator which short-circuits on error. In this sense, the thrown exception model is nice since it doesn't clutter the `Config{}` construction.

```c++
// Potential parse() if std::stof returned std::expected<float, E>
auto parse(int argc, char* argv[]) -> std::expected<Config, parse_error> {
    if (argc != 3) return std::unexpected(parse_error::WRONG_NUMBER);
    float a, b;
    if(!(a = std::stof_exp(argv[1]))) return std::unexpected<parse_error::INVALID_FLOAT>;
    if(!(b = std::stof_exp(argv[2]))) return std::unexpected<parse_error::INVALID_FLOAT>;

    return Config{a, b};
}
```

## `rusty.cc`

```bash
make rusty
./rusty
```

The `rusty.h` header file defines typedefs and functions to make C++ exceptions and optionals handle like Rust's Result and Option. I wrote `match()` statements for both types which resemble Rust's `match`, though imperfectly.

/// @author Blake Freer
/// @date 2024-10-23

#include <expected>
#include <format>
#include <iostream>
#include <optional>

typedef struct {
    float a;
    float b;
} Config;

enum class parse_error { WRONG_NUMBER, INVALID_FLOAT };

// Wrap std::stof to catch exceptions and return std::expected
auto stof_exp(const std::string& str) -> std::expected<float, parse_error> {
    try {
        return std::stof(str);
    } catch (const std::invalid_argument& e) {
        return std::unexpected(parse_error::INVALID_FLOAT);
    } catch (const std::out_of_range& e) {
        return std::unexpected(parse_error::INVALID_FLOAT);
    }
}

/**
 * @brief Parse CLI args into a Config struct
 */
auto parse(int argc, char* argv[]) -> std::expected<Config, parse_error> {
    if (argc != 2 + 1) return std::unexpected(parse_error::WRONG_NUMBER);

    /* If C++ had Rust's ? operator, the rest of this function could be
     * replaced by:

    return Config{
        stof_exp(argv[1])?,
        stof_exp(argv[2])?,
    };
    */

    auto a = stof_exp(argv[1]);
    if (!a) return std::unexpected(a.error());
    auto b = stof_exp(argv[2]);
    if (!b) return std::unexpected(b.error());

    return Config{
        a.value(),
        b.value(),
    };
}

/**
 * @brief Divide two floats. Return NULLOPT if divisor is 0.
 */
auto divide(float a, float b) -> std::optional<float> {
    if (b == 0) return std::nullopt;
    return a / b;  // is cast to optional<float> during return
}

int main(int argc, char* argv[]) {
    const auto _config = parse(argc, argv);

    if (!_config.has_value()) {
        switch (_config.error()) {  // Handle error by type.
            case parse_error::WRONG_NUMBER:
                std::cerr << "Error: Expected 2 arguments." << std::endl;
                break;
            case parse_error::INVALID_FLOAT:
                std::cerr << "Error: Invalid arg type, expected float."
                          << std::endl;
                break;
        }
        return EXIT_FAILURE;
    }

    Config config = _config.value();
    std::cout << std::format("Calculating {} / {}", config.a, config.b)
              << std::endl;

    const std::optional<float> val = divide(config.a, config.b);

    /*
     * The following results are logically equivalent but demonstrate different
     * ways of processing an optional value
     */

    // check val.has_value() -> most direct, most verbose
    auto result1 = [val]() {
        if (val.has_value()) {
            return std::format(" = {:.3f}", val.value());
        } else {
            return std::string{"Divide by 0 error."};
        }
    }();
    std::cout << result1 << std::endl;

    // val is ~True if is not nullopt (even if its float value is 0.0)
    auto result2 = [val]() {
        return (val ? std::format(" = {:.3f}", val.value())
                    : "Divide by 0 error.");
    }();
    std::cout << result2 << std::endl;

    // Transform `val`
    auto result3 = [val]() {
        return val
            .transform([](float i) { return std::format(" = {:.3f}", i); })
            .value_or("Divide by 0 error.");
    }();
    std::cout << result3 << std::endl;

    return EXIT_SUCCESS;
}
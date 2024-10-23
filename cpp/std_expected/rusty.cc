/// @author Blake Freer
/// @date 2024-10-23

#include <format>
#include <iostream>

#include "rusty.h"

auto divide_option(float a, float b) -> rusty::Option<float> {
    if (b == 0) return rusty::None;
    return a / b;
}

auto divide_result(float a, float b) -> rusty::Result<float, std::string> {
    if (b == 0) return rusty::Err("Divide by 0 error.");
    return a / b;
}

int main() {
    std::cout << "--- OPTION ---" << std::endl;

    // match() returns string which is printed externally
    auto option1 = rusty::match<float, std::string>(
        divide_option(1, 4),
        [](float f) { return std::format("Divide resulted in {:.3f}", f); },
        []() { return std::string{"Divide by 0 error."}; });
    std::cout << option1 << std::endl;

    // match() returns void, printing is internal
    rusty::match<float, void>(
        divide_option(1, 0),
        [](float f) {
            std::cout << std::format("Divide resulted in {:.3f}", f)
                      << std::endl;
        },
        []() { std::cout << "Divide by 0 error." << std::endl; });

    std::cout << "--- RESULT ---" << std::endl;

    // match() returns string which is printed externally
    auto result1 = rusty::match<float, std::string, std::string>(
        divide_result(1, 5),
        [](auto f) { return std::format("Result = {:.3f}", f); },
        [](auto e) { return e; });
    std::cout << result1 << std::endl;

    // match() returns void, printing is internal
    rusty::match<float, std::string, void>(
        divide_result(1, 0),
        [](auto f) {
            std::cout << std::format("Result = {:.3f}", f) << std::endl;
        },
        [](auto e) { std::cout << e << std::endl; });

    return EXIT_SUCCESS;
}
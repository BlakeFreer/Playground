/// @author Blake Freer
/// @date 2024-03-19

#include <iostream>

int main(int argc, char* argv[]) {
    std::cout << "Hello World";

    for (size_t i = 1; i < argc; i++) {
        std::cout << ", " << argv[i];
    }

    std::cout << std::endl;

    return 0;
}

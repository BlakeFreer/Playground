/// @author Blake Freer
/// @date 2024-03-19
/// @brief Subtract two command line arguments

#include <iostream>

#include "mymath.h"

int main(int argc, char* argv[]) {
    if (argc != 3) {
        std::cerr << "Expected 2 integer command line arguments.";
        return 1;
    }

    int x = strtol(argv[1], 0, 0);
    int y = strtol(argv[2], 0, 0);

    std::cout << "Subtracting ";
    std::cout << x << " - " << y << " = " << mymath::subtract(x, y)
              << std::endl;

    return 0;
}
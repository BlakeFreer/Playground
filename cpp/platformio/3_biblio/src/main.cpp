#include <iostream>

#include "calc.hpp"

int main(void) {
    std::cout << "Hello World" << std::endl;

    std::cout << calc::add(1, 2) << std::endl;
    std::cout << calc::multiply(3, 2) << std::endl;

    return 0;
}
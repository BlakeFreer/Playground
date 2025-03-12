#include <iostream>

#include "bindings.hpp"

int main(int argc, char* argv[]) {
    bindings::light.Set(true);
    return 0;
}
#pragma once

#include <iostream>

#include "../shared.hpp"

namespace cli {

class DigitalOut : public shared::DigitalOut<DigitalOut> {
public:
    DigitalOut(std::string name)
        : shared::DigitalOut<DigitalOut>(*this), name_(name) {}

    void InnerSet(bool value) {
        std::cout << "CLI: Digital Out is " << value << std::endl;
    }

private:
    std::string name_;
};

}  // namespace cli

#pragma once

#include <iostream>

#include "../shared.hpp"

namespace stm {

class DigitalOut : public shared::DigitalOut<DigitalOut> {
public:
    DigitalOut(int pin, int port) : pin_(pin), port_(port) {}

    void InnerSet(bool value) {
        std::cout << "STM: Set Port(" << port_ << ") Pin(" << pin_ << ") to "
                  << value << std::endl;
    }

private:
    int pin_;
    int port_;
};

}  // namespace stm
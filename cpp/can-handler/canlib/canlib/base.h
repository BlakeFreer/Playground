#pragma once

#include <iostream>

#include "format.h"
#include "msg.h"

namespace can {

class Bus;

class Base {
   public:
    void send(const RawMessage& msg) {
        std::cout << std::format("Sending message: {}", msg) << std::endl;
    }

   private:
    void register_bus(Bus* bus) {
        bus_ = bus;
    }

   private:
    Bus* bus_;
    friend class Bus;
};

}  // namespace can
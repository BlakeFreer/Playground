#include <ctime>
#include <format>
#include <iostream>

#include "canlib/base.h"
#include "canlib/bus.h"
#include "canlib/format.h"
#include "canlib/msg.h"
#include "generated/can_bus.h"
#include "generated/can_msgs.h"

class StmCAN : public can::Base {
   public:
    void Send(const can::RawMessage& msg) override {
        std::cout << std::format("STM | Sending message: {}", msg) << std::endl;
    }

   private:
    uint32_t GetTimestamp() override {
        return std::time(NULL);
    }
};

void print_contactor_state(const can::ContactorStateMsg& msg,
                           uint32_t timestamp) {
    std::cout << std::format("{} | Pos: {}, Pre: {}, Neg: {}", timestamp,
                             msg.pack_positive, msg.pack_precharge,
                             msg.pack_negative)
              << std::endl;
}

class Nonconstructable {
    Nonconstructable() = delete;
};

int main() {
    StmCAN base{};  // this would usually go in bindings
    can::VehBus veh_bus{base};

    can::PackStateMsg msg1{
        .pack_current = 1.0,
        .pack_inst_voltage = 2.0,
        .avg_cell_voltage = 3.0,
        .populated_cells = 4,
    };

    can::ContactorStateMsg msg2{
        .pack_positive = 1,
        .pack_precharge = 0,
        .pack_negative = 1,
    };

    veh_bus.Send(msg1);
    veh_bus.Send(msg2);

    // Nothing received yet -> should print No Value
    auto latest = veh_bus.GetLatest<can::ContactorStateMsg>();
    if (latest.has_value()) {
        auto [msg, timestamp] = latest.value();  // unpack the optional
        print_contactor_state(msg, timestamp);
    } else {
        std::cout << "No value" << std::endl;
    }

    // This would usually be called in the CAN interrupt.
    base.Receive(can::ContactorStateMsg{10, 5, 19}.encode());

    // Now there should be a value -> we don't even need to call update
    latest = veh_bus.GetLatest<can::ContactorStateMsg>();
    if (latest.has_value()) {
        auto [msg, timestamp] = latest.value();  // unpack the optional
        print_contactor_state(msg, timestamp);
    } else {
        std::cout << "No value" << std::endl;
    }

    return 0;
}

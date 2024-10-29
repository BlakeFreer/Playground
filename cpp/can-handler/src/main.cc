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
        std::cout << std::format("Sending message: {}", msg) << std::endl;
    }
};

void print_contactor_state(const can::ContactorStateMsg& msg) {
    std::cout << std::format("Pos: {}, Pre: {}, Neg: {}", msg.pack_positive,
                             msg.pack_precharge, msg.pack_negative)
              << std::endl;
}

int main() {
    auto base = StmCAN{};

    auto bus = can::VehBus{base};

    auto msg1 = can::PackStateMsg{
        .pack_current = 1.0,
        .pack_inst_voltage = 2.0,
        .avg_cell_voltage = 3.0,
        .populated_cells = 4,
    };

    auto msg2 = can::ContactorStateMsg{
        .pack_positive = 1,
        .pack_precharge = 0,
        .pack_negative = 1,
    };

    bus.Send(msg1.encode());
    bus.Send(msg2.encode());

    // Nothing received yet -> should print No Value
    auto latest = bus.GetLatest<can::ContactorStateMsg>();
    if (latest.has_value()) {
        auto msg = latest.value();  // unpack the optional
        print_contactor_state(msg);
    } else {
        std::cout << "No value" << std::endl;
    }
    // This would usually be called in the CAN interrupt.
    base.Receive(can::ContactorStateMsg{10, 5, 19}.encode());

    // Now there should be a value
    latest = bus.GetLatest<can::ContactorStateMsg>();
    if (latest.has_value()) {
        auto msg = latest.value();  // unpack the optional
        print_contactor_state(msg);
    } else {
        std::cout << "No value" << std::endl;
    }

    return 0;
}

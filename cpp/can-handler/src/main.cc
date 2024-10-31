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
        // Each CanBase has to provide a timestamp function, but it is called
        // automatically by CanBase.Receive
        return std::time(NULL);
    }
};

void print_contactor_state(const can::RxContactorState& msg) {
    std::cout << std::format("{} | Pos: {} Pre: {} Neg: {}", msg.Timestamp(),
                             msg.PackPositive(), msg.PackPrecharge(),
                             msg.PackNegative())
              << std::endl;
}

/* The output of main() is
STM | Sending message: [100] 01 A2 B3 C4 D5 E6 F7
STM | Sending message: [200] 01 00 01
Invalid PackState parameters.
No value
1730338897 | Pos: 1 Pre: 0 Neg: 1
*/

int main() {
    StmCAN base{};  // this would usually go in bindings
    can::VehBus veh_bus{base};

    /***************************************************************
        Tx Example
    ***************************************************************/

    auto msg1 = can::TxPackState::create(1.0, 2.0, 3.0, 4.0);
    if (msg1.has_value()) {
        veh_bus.Send(msg1.value());
    } else {
        std::cerr << "Invalid PackState parameters." << std::endl;
    }

    // This IF statement ensures that ::create returns a Msg object, since it
    // can return NULLOPT if the parameters are invalid. We could remove this
    // part by allowing the user to create a Msg object with invalid parameters,
    // but I think that would be worse.

    // Going the other way, we could provide MORE information when ::create
    // fails by using std::expected and returning an enum value indicating which
    // parameter was invalid. This would look like
    // `std::expected<can::TxPackState, can::PackStateError>` and could be
    // checked with a switch statement like
    // ```
    // auto msg = can::TxPackState::create(1.0, 2.0, 3.0, 4.0);
    // if (msg.has_value()) {
    //     veh_bus.Send(msg.value());
    // } else {
    //     switch (msg.error()) {
    //         case can::PackStateError::kCurrentNegative:
    //             std::cerr << "Current cannot be negative." << std::endl;
    //             return 1;
    //         case can::PackStateError::kInstVoltageNegative:
    //             std::cerr << "Inst voltage cannot be negative." << std::endl;
    //             return 1;
    //         case can::PackStateError::kAvgCellVoltageNegative:
    //             std::cerr << "Avg cell voltage cannot be negative." <<
    //             std::endl; return 1;
    //         case can::PackStateError::kPopulatedCellsNegative:
    //             std::cerr << "Populated cells cannot be negative." <<
    //             std::endl; return 1;
    //     }
    // }
    // ```
    // Is this verbose? Yes. But we are writing an embedded system. I don't want
    // to send a ContactorsMessage with status=2 when it should be either 0
    // or 1. Who knows how the BMS would respond.
    //
    // Note that you don't need to use the whole switch case; you could perform
    // a generic action in the else block.

    auto msg2 = can::TxContactorState::create(1, 0, 1);
    if (msg2.has_value()) {
        veh_bus.Send(msg2.value());
    } else {
        std::cerr << "Invalid ContactorState parameters." << std::endl;
    }

    // Invalid example
    auto msg3 = can::TxPackState::create(-1.0, 2.0, 3.0, 4.0);
    if (msg3.has_value()) {
        veh_bus.Send(msg3.value());
    } else {
        // this should run since pack_current is negative
        std::cerr << "Invalid PackState parameters." << std::endl;
    }

    /***************************************************************
        Rx Example
    ***************************************************************/

    auto latest = veh_bus.GetLatest<can::RxContactorState>();

    // Nothing received yet -> should print No Value
    if (latest.has_value()) {
        auto msg = latest.value();  // unpack the optional
        print_contactor_state(msg);
    } else {
        std::cout << "No value" << std::endl;
    }

    // This would usually be called in the CAN interrupt.
    uint8_t sample_data[3] = {1, 0, 1};
    base.Receive(can::RawMessage::New(0x200, 3, sample_data));

    // Now there should be a value. We never needed to call Update()
    latest = veh_bus.GetLatest<can::RxContactorState>();
    if (latest.has_value()) {
        auto msg = latest.value();  // unpack the optional
        print_contactor_state(msg);
    } else {
        std::cout << "No value" << std::endl;
    }

    return 0;
}

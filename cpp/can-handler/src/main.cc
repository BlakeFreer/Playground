#include <format>
#include <iostream>

#include "canlib/base.h"
#include "canlib/bus.h"
#include "canlib/format.h"
#include "canlib/msg.h"
#include "gen_can.h"

int main() {
    auto base = can::Base{};

    auto bus = can::VehBus{base};

    auto msg1 = can::PackState{
        .pack_current = 1.0,
        .pack_inst_voltage = 2.0,
        .avg_cell_voltage = 3.0,
        .populated_cells = 4,
    };

    auto msg2 = can::ContactorState{
        .pack_positive = 1,
        .pack_precharge = 0,
        .pack_negative = 1,
    };

    bus.Send(msg1);
    bus.Send(msg2);
    bus.GetLatest<can::PackState>();
    return 0;
}

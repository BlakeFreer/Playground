#include "canlib/msg.h"

namespace can {

namespace msg {
enum _id {
    PackState = 1572,
    ContactorState = 200,
};
}  // namespace msg

template <>
struct CanMsg<msg::PackState> {
    float pack_current;
    float pack_inst_voltage;
    float avg_cell_voltage;
    uint8_t populated_cells;

    auto Pack() -> RawMessage const {
        // should actually pack bits here
        uint8_t data[7] = {0x01, 0xA2, 0xB3, 0xC4, 0xD5, 0xE6, 0xF7};
        return RawMessage::New(msg::PackState, 7, data);
    };

    static auto Unpack(const RawMessage& msg) -> CanMsg<msg::PackState> {
        return CanMsg<msg::PackState>{
            .pack_current = (float)msg.data[0],
            .pack_inst_voltage = (float)msg.data[1],
            .avg_cell_voltage = (float)msg.data[2],
            .populated_cells = msg.data[3],
        };
    }
};

template <>
struct CanMsg<msg::ContactorState> {
    uint8_t pack_positive;
    uint8_t pack_precharge;
    uint8_t pack_negative;
};

}  // namespace can
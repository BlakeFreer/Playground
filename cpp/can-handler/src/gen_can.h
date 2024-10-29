#include "canlib/bus.h"
#include "canlib/msg.h"

namespace can {

struct PackState {
    float pack_current;
    float pack_inst_voltage;
    float avg_cell_voltage;
    uint8_t populated_cells;

    static constexpr auto id() -> uint32_t {
        return 0x100;
    }
    static constexpr auto data_length() -> uint8_t {
        return 7;
    }

    auto encode() const -> RawMessage {
        // should actually pack bits here
        uint8_t data[7] = {0x01, 0xA2, 0xB3, 0xC4, 0xD5, 0xE6, 0xF7};
        return RawMessage::New(PackState::id(), PackState::data_length(), data);
    };

    static auto decode(const RawMessage& msg) -> PackState {
        return PackState{
            .pack_current = (float)msg.data[0],
            .pack_inst_voltage = (float)msg.data[1],
            .avg_cell_voltage = (float)msg.data[2],
            .populated_cells = msg.data[3],
        };
    }
};

struct ContactorState {
    uint8_t pack_positive;
    uint8_t pack_precharge;
    uint8_t pack_negative;

    static constexpr auto id() -> uint32_t {
        return 0x200;
    }

    static constexpr auto data_length() -> uint8_t {
        return 3;
    }

    auto encode() const -> RawMessage {
        uint8_t data[3] = {1, 0, 1};
        return RawMessage::New(ContactorState::id(),
                               ContactorState::data_length(), data);
    }

    static auto decode(const RawMessage& msg) -> ContactorState {
        return ContactorState{
            .pack_positive = msg.data[0],
            .pack_precharge = msg.data[1],
            .pack_negative = msg.data[2],
        };
    }
};

class VehBus : public Bus {
   public:
    VehBus(Base& can_base) : Bus(can_base) {}

    template <>
    PackState GetLatest<PackState>() const {
        return pack_state_;
    }

   private:
    PackState pack_state_;
    ContactorState constactor_state_;

    void AddMessage(const RawMessage& msg) override {
        if (msg.id == PackState::id()) {
            auto pack_state = PackState::decode(msg);
            std::cout << "Received PackState: "
                      << "pack_current: " << pack_state.pack_current
                      << ", pack_inst_voltage: " << pack_state.pack_inst_voltage
                      << ", avg_cell_voltage: " << pack_state.avg_cell_voltage
                      << ", populated_cells: " << pack_state.populated_cells
                      << std::endl;
        } else if (msg.id == ContactorState::id()) {
            auto contactor_state = ContactorState::decode(msg);
            std::cout << "Received ContactorState: "
                      << "pack_positive: " << contactor_state.pack_positive
                      << ", pack_precharge: " << contactor_state.pack_precharge
                      << ", pack_negative: " << contactor_state.pack_negative
                      << std::endl;
        }
    }
};

}  // namespace can
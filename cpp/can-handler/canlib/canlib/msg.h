#pragma once

#include <concepts>
#include <cstdint>
#include <cstring>

namespace can {

struct RawMessage {
    uint32_t id;
    uint8_t data_length;
    uint8_t data[8];

    /**
     * Create a RawMessage. Could also be a constructor.
     */
    static RawMessage New(uint32_t id, uint8_t data_length,
                          const uint8_t data[8]) {
        RawMessage m{
            .id = id,
            .data_length = data_length,
        };
        std::memcpy(m.data, data, data_length);
        return m;
    }
};

// TxMessage concept is required by Bus.Send()
template <typename T>
concept TxMessage = requires(const T msg) {
    { msg.encode() } -> std::same_as<RawMessage>;
};

// Don't need an Rx message concept as no function needs to take in an arbitrary
// Rx message.

}  // namespace can

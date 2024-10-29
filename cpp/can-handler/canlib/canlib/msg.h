#pragma once

#include <concepts>
#include <cstdint>
#include <cstring>

namespace can {

struct RawMessage {
    uint32_t id;
    uint8_t data_length;
    uint8_t data[8];

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

template <typename T>
concept Message = requires(const T msg) {
    { msg.encode() } -> std::same_as<RawMessage>;
    { T::decode(std::declval<RawMessage>()) } -> std::same_as<T>;
    { T::id() } -> std::same_as<uint32_t>;
    { T::data_length() } -> std::same_as<uint8_t>;
};

}  // namespace can

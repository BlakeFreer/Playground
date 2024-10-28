#pragma once

#include <concepts>
#include <cstdint>
#include <cstring>
#include <format>

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
concept Message = requires(T msg) {
    { msg.encode() } -> std::same_as<RawMessage>;
    { T::decode(std::declval<RawMessage>()) } -> std::same_as<T>;
    { T::id() } -> std::same_as<uint32_t>;
    { T::data_length() } -> std::same_as<uint8_t>;
};

}  // namespace can

template <>
struct std::formatter<can::RawMessage> {
    constexpr auto parse(std::format_parse_context& ctx) {
        // could determine if user passed a format option
        // ex :X to print hex, :D to print decimal
        return ctx.begin();
    }

    auto format(const can::RawMessage& msg, std::format_context& ctx) const {
        std::format_to(ctx.out(), "[{:X}]", msg.id);

        for (int i = 0; i < msg.data_length; i++) {
            std::format_to(ctx.out(), " {:02X}", msg.data[i]);
        }
        return ctx.out();
    }
};

template <can::Message T>
struct std::formatter<T> {
    constexpr auto parse(std::format_parse_context& ctx) {
        // could determine if user passed a format option
        // ex :X to print hex, :D to print decimal
        return ctx.begin();
    }

    auto format(const T& msg, std::format_context& ctx) const {
        return std::format_to(ctx.out(), "[{:X}] has length {}", T::id(),
                              T::data_length());
    }
};
#pragma once

#include <format>

#include "msg.h"

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
#pragma once

#include <optional>
#include <tuple>

#include "etl/deque.h"
#include "msg.h"

namespace can {

enum class QueueDirection { FIFO, LIFO };

template <size_t capacity, QueueDirection direction>
class MsgQueue {
   public:
    auto push(RawMessage msg) -> void;
    auto pop() -> std::optional<std::tuple<RawMessage, uint32_t>>;
    auto peek() -> std::optional<const std::reference_wrapper<RawMessage>>;

   private:
    etl::deque<RawMessage, capacity> queue_;
    uint32_t dropped_messages_;
};

template <size_t capacity, QueueDirection dir>
auto MsgQueue<capacity, dir>::pop()
    -> std::optional<std::tuple<RawMessage, uint32_t>> {
    if (queue_.empty()) return std::nullopt;

    auto t = dropped_messages_;
    dropped_messages_ = 0;
    return std::tuple(queue_.pop_front(), t);
}

template <size_t capacity, QueueDirection dir>
void MsgQueue<capacity, dir>::push(RawMessage msg) {
    if constexpr (dir == QueueDirection::FIFO) {
        if (queue_.full()) {
            dropped_messages_++;
            // drop incoming message
        } else {
            queue_.push_back(msg);
        }
    } else {
        if (queue_.full()) {
            dropped_messages_++;
            queue_.pop_back();
            // drop oldest message
        }
        queue_.push_front(msg);
    }
}

template <size_t capacity, QueueDirection dir>
auto MsgQueue<capacity, dir>::peek()
    -> std::optional<const std::reference_wrapper<RawMessage>> {
    if (queue_.empty()) return std::nullopt;
    return queue_.front();
}

}  // namespace can

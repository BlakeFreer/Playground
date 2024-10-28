#include <format>
#include <iostream>

#include "can-messages.h"
#include "canlib/mailbox.h"
#include "canlib/msg.h"

int main() {
    auto m1 = can::CanMsg<can::msg::PackState>{};

    auto raw = m1.Pack();
    std::cout << std::format("{}", raw) << std::endl;

    auto m2 = can::CanMsg<can::msg::PackState>::Unpack(raw);

    auto mb = can::MsgQueue<3, can::QueueDirection::FIFO>{};
    auto mb2 = can::MsgQueue<3, can::QueueDirection::LIFO>{};

    mb.push(raw);
    mb2.push(raw);

    const auto m_ = mb.peek();
    if (m_.has_value()) {
        auto m = m_.value().get();
        m.id = 10;
        std::cout << std::format("{}", m) << std::endl;
    }

    std::cout << std::format("{}", raw) << std::endl;

    return 0;
}
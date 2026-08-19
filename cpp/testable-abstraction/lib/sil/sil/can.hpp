#pragma once

#include <queue>

#include "periph.hpp"

namespace macfe::sil {

class Can : public periph::Can {
public:
    Can() : messages_(std::queue<CanMessage>()) {}

    void Send(const CanMessage& msg) override {
        messages_.push(msg);
    }

    CanMessage Read(void) {
        CanMessage msg = messages_.front();
        messages_.pop();
        return msg;
    }

private:
    std::queue<CanMessage> messages_;
};

}  // namespace macfe::sil
#pragma once

#include <queue>

#include "abstraction.hpp"

namespace macfe::sil {

class Can : public abstraction::Can {
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
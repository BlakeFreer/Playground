#pragma once

#include <string>

namespace macfe {

class CanMessage {
public:
    CanMessage(std::string contents_) : contents(contents_) {}

    std::string contents;  // example only. real CAN messages aren't strings
};

}  // namespace macfe

namespace macfe::periph {

class AnalogInput {
public:
    virtual float ReadVoltage() = 0;
};

class Can {
public:
    virtual void Send(const CanMessage& msg) = 0;
};

}  // namespace macfe::periph
#pragma once

#include "abstraction.hpp"

namespace macfe::sil {

class AnalogInput : public abstraction::AnalogInput {
public:
    AnalogInput() : voltage_(0.0f) {}

    float ReadVoltage() override {
        return voltage_;
    }

    // A simple, local SIL could set this variable to change
    // result of the next `ReadVoltage()`
    void SetVoltage(float voltage) {
        voltage_ = voltage;
    }

private:
    float voltage_;
};

}  // namespace macfe::sil
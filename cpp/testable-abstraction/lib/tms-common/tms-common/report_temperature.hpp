#pragma once

#include "periph.hpp"

using namespace macfe::periph;

namespace macfe::tms {

void report_temperatures(AnalogInput& temperature_sensor, Can& can_bus);

// helpers. exposed for unit testing
namespace priv {

float volt_to_temperature_degc(float volt);
CanMessage package_can_message(float temperature_degc);

}  // namespace priv

}  // namespace macfe::tms
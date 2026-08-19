#include "report_temperature.hpp"

#include "periph.hpp"

using namespace macfe::periph;

namespace macfe::tms {

namespace priv {
float volt_to_temperature_degc(float volts) {
    // dummy conversion
    return volts * 10.0f;
}

CanMessage package_can_message(float temperature_degc) {
    char buffer[32];
    std::sprintf(buffer, "temperature=%.1fC", temperature_degc);
    return CanMessage(std::string(buffer));
}
}  // namespace priv

// SIL test this
void report_temperatures(AnalogInput& temperature_sensor, Can& can_bus) {
    float volts = temperature_sensor.ReadVoltage();
    float temperature = priv::volt_to_temperature_degc(volts);
    CanMessage message = priv::package_can_message(temperature);
    can_bus.Send(message);
}

}  // namespace macfe::tms
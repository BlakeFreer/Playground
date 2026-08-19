#include <iostream>
#include <string>

#include "abstraction.hpp"
#include "sil/analog_input.hpp"
#include "sil/can.hpp"
#include "tms-common/report_temperature.hpp"

using namespace macfe;

int main(void) {
    std::cout << "------ TMS SIL ------" << std::endl;

    // Define peripherals needed for this test
    sil::AnalogInput temperature_sensor{};
    sil::Can can_bus{};

    // Set inputs
    temperature_sensor.SetVoltage(2.5f);

    // Run code
    tms::report_temperatures(temperature_sensor, can_bus);

    // Check outputs
    CanMessage output = can_bus.Read();
    std::string expected = "temperature=25C";
    if (output.contents == expected) {
        std::cout << "Test success" << std::endl;
        return 0;
    } else {
        std::cout << "Test failure" << std::endl;
        std::cout << "Expected: " << expected << std::endl;
        std::cout << "Actual:   " << output.contents << std::endl;
        return 1;
    }
}
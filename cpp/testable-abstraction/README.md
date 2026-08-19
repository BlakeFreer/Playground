# Testable Abstraction

The Mac Formula racecar repository has a massive hardware abstraction system. It was designed to allow the entire system to run on any platform, primarily vehicles' STM32 microcontrollers or a SIL.

The abstraction system is too pervasive. It wraps low-level IO types in a `bindings.cc`, far from the App layer, preventing the App from using STM32-specific features. Timers and interrupts are both exluded, but would be invaluable for a real time system.

The abstractions were performed in the name of SIL testability: If the entire system can be mocked, then surely the entire system can be tested. This is goal is too broad. A SIL does not need to run the entire system. That is what a HIL, running STM32 firmare, is for. SIL should lie between unit tests and HIL integration tests.

Consider the TMS and its hypothetical `report_temperature()` task:

1. Read temperature sensor values using an ADC.
2. Convert raw ADC values to a temperature.
3. Package the temperatures into a CAN message.
4. Transmit the message over CAN.

Steps 2 and 3 can be unit tested in isolation. Steps 1 and 4 involve IO and need a larger setup for testing. This is where SIL comes in.

With unit tests proving that 2 and 3 succeed, SIL should test the entire task, from setting a sensor voltage to reading the CAN message. The TMS app code should call the `report_temperatures()` method inside FreeRTOS. `report_temperatures()` should accept platform-abstracted arguments for the ADC and CAN, allowing the method to be called by the SIL, using SIL IO, without dragging the entire RTOS into SIL.

```c++
// Peripheral abstraction interfaces
class AdcInterface;
class CanInterface;

// Generic TMS method
void report_temperatures(AdcInterface sensor, CanInterface can) {
    ...
}

// Stm implementation (TMS App code)
class Stm32Adc(AdcInterface);
class Stm32Can(CanInterface);

void freertos_task(void*) {
    auto temperature_sensor = Stm32Adc();
    auto can_bus = Stm32Can();

    while(true) {
        report_temperatures(temperature_sensor, can_bus);

        // call some platform specific code
        stm_timer_wait_ms(10);
    }
}

// SIL implemenation (SIL Code)
class SilAdc(AdcInterface);
class SilCan(CanInterface);

void test_report_temperatures() {
    // set up mock IO
    auto temperature_sensor = SilAdc();
    temperature_sensor.set_voltage(3.2);

    auto can_bus = SilCan();

    report_temperatures(mock_temperature_sensor, mock_can_bus);
    
    assert(can_bus.receive() == TmsMessage{...});
}
```


Further, the extreme platform abstraction limited both Firmware and SIL development. FreeRTOS cannot easily execute on a Linux machine, so most of the boards were non-SIL compliant. A board could either benefit from FreeRTOS or be SIL compliant. We took efforts to port FreeRTOS to Linux, but this is was a major distraction. The class of bugs associated with FreeRTOS (stack overflows, race conditions, task overrum) will not surface on a FreeRTOS simulation. They are inherently hardware specific. The failed FreeRTOS port became a SIL inhibitor.

Both the FreeRTOS task and SIL test can call `report_temperatures()` using their implementation of the peripheral interfaces. This maintains testability of core code, but limits the restrictions of platform abstraction to specific methods. The STM32 code can use platform specific code anywhere outside of the tested method.

By creating smaller region of abstracted code, it also becomes easier to write SIL tests. Previously, the entire system was packaged as one monolith, capable of running on STM or SIL, but orchestrating dozens of SIL IO to run the entire system becomes a challenge of its own. Smaller methods, like `report_temperatures()`, only require a couple IO which are easy to set up. This makes SIL testing natural and easy, increasing adoption.

Again, full system testing belongs to the HIL, not SIL.

## This directory

I will mock up a platform abstraction system which includes STM32 and SIL code. It will simulate the TMS functionality, exposing a testable `report_tempertures` function while still using FreeRTOS and STM32 methods in its `main`.

## Background

This style of abstraction is used in the Rust [embedded-hal](https://docs.rs/embedded-hal/latest/embedded_hal/) ecosystem.

## Other benefits of a refactor

### Less code nesting

The current board directories are extremely nested. Each projects' platform configuration is hidden at `PROJECT/src/platforms/PLATFORM/bindings.cc`. Consider LV Controller's directory below.

```
$ tree racecar/projects/lvcontroller -d -L4
.
├── include
│   ├── bindings.hpp
│   └── generated
├── lib
├── platformio.ini
└── src
    ├── accumulator
    ├── brakelight
    ├── dcdc
    ├── fans
    ├── main.cc
    ├── mcal -> ../../../lib/mcal/
    ├── motor_controller
    ├── platforms
    │   ├── cli
    │   │   └── bindings.cc
    │   ├── linux
    │   │   ├── bindings.cc
    │   │   └── vcan_setup.sh
    │   ├── sil
    │   │   ├── bindings.cc
    │   │   └── proto
    │   └── stm32-ev6
    │       ├── bindings.cc
    │       ├── Inc
    │       ├── lvcontroller.ioc
    │       ├── post_cubemx.sh
    │       ├── Src
    │       ├── Startup
    │       └── STM32F767ZITX_FLASH.ld
    ├── scheduler
    ├── suspension
    └── tssi
```

With the proposed simpler abstraction system, the entire `project/lvcontroller` is assumed to run on STM32. Testable logic is pulled into a separate `lvcontroller-lib` folder which can be compiled into firmware or into a SIL binary. The simplified directory structure may look like:

```
.
├── Inc
│   └── generated
├── lib
│   └── lvcontroller-lib
│       ├── accumulator
│       ├── brakelight
│       ├── dcdc
│       ├── fans
│       ├── motor_controller
│       ├── suspension
│       └── tssi
├── Src
│   └── main.cc
├── lvcontroller.ioc
├── platformio.ini
├── post_cubemx.sh
└── STM32F767ZITX_FLASH.ld
```

This project clearly runs exclusively on STM32. Platform-agnostic behaviour is pulled out to `lvcontroller-lib` which can be tested by a separate SIL binary.

There are no more `bindings.hpp` or `bindings.cc` contracts since the code only needs to run on STM. Platform-abstracted logic defines a small-scale peripheral contract via the function signature.
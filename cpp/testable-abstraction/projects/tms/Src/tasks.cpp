#include <cstdint>

#include "abstraction.hpp"
#include "stm32/analog_input.hpp"
#include "stm32/can.hpp"
#include "tms-common/report_temperature.hpp"

// CubeMX
#include "adc.h"
#include "can.h"
#include "main.h"

// FreeRTOS
#include "FreeRTOS.h"
#include "task.h"

extern "C" {
void start_rtos(void);
}

static const size_t STACK_SIZE_WORDS = 2048 * 16;
static const uint32_t PRIORITY_10HZ = 1;

StaticTask_t t10hz_control_block;
StackType_t t10hz_buffer[STACK_SIZE_WORDS];

void task_10hz(void* argument) {
    (void)argument;

    const uint32_t kUpdatePeriodMs = 100;
    TickType_t wake_time = xTaskGetTickCount();

    auto adc = macfe::stm32f::AnalogInput(&hadc1, ADC_CHANNEL_0, 3.3f);
    auto can = macfe::stm32f::CanBase(&hcan2);

    while (true) {
        macfe::tms::report_temperatures(adc, can);

        vTaskDelayUntil(&wake_time, pdMS_TO_TICKS(kUpdatePeriodMs));
    }
}

void start_rtos(void) {
    xTaskCreateStatic(task_10hz, "10HZ", STACK_SIZE_WORDS, NULL, PRIORITY_10HZ,
                      t10hz_buffer, &t10hz_control_block);

    vTaskStartScheduler();
}
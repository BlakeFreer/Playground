#include "gtest/gtest.h"
#include "tms.hpp"

using namespace macfe::tms::priv;

TEST(Tms, TemperatureToVolt) {
    EXPECT_EQ(volt_to_temperature_degc(1.0f), 10.0f);
    EXPECT_EQ(volt_to_temperature_degc(1.8f), 18.0f);
    EXPECT_EQ(volt_to_temperature_degc(2.5f), 25.0f);
}

TEST(Tms, CanMessageFormat) {
    CanMessage msg = package_can_message(13.8f);
    EXPECT_STREQ(msg.contents.c_str(),
                 std::string("temperature=13.8C").c_str());
}

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
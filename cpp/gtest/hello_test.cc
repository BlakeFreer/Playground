#include <gtest/gtest.h>

TEST(HelloTest, BasicAssertions) {
    // Two strings should not be equal
    EXPECT_STRNE("hello", "world");

    // Expect equality
    EXPECT_EQ(7 * 6, 42);

    EXPECT_EQ(1, 0);
}
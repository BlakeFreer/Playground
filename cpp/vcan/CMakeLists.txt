cmake_minimum_required(VERSION 3.0)
project(vcan)

# Specify the C++ standard
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_EXPORT_COMPILE_COMMANDS 1)

# Add executable targets
add_executable(send send.cc)
add_executable(receive receive.cc)

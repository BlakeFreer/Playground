add_library(canlib INTERFACE)
target_link_libraries(canlib INTERFACE etl)

target_include_directories(canlib INTERFACE ${CMAKE_CURRENT_SOURCE_DIR})
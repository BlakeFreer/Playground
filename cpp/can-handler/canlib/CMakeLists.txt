add_library(canlib)
target_link_libraries(canlib PRIVATE etl)

target_include_directories(canlib PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_sources(canlib PRIVATE src/base.cc src/bus.cc)
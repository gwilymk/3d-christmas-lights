cmake_minimum_required(VERSION 3.13)

# initialize pico-sdk from GIT
# (note this can come from environment, CMake cache etc)
set(PICO_SDK_FETCH_FROM_GIT on)

# pico_sdk_import.cmake is a single file copied from this SDK
# note: this must happen before project()
include(pico_sdk_import.cmake)

project(ChristmasTreeLights)

# initialize the Raspberry Pi Pico SDK
pico_sdk_init()

add_executable(ChristmasTreeLights)

target_sources(ChristmasTreeLights PRIVATE main.c)
pico_generate_pio_header(ChristmasTreeLights ${CMAKE_CURRENT_LIST_DIR}/ws2812.pio)

# Add pico_stdlib library which aggregates commonly used features
target_link_libraries(ChristmasTreeLights pico_stdlib hardware_pio)


# create map/bin/hex/uf2 file in addition to ELF.
pico_add_extra_outputs(ChristmasTreeLights)
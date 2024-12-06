cmake_minimum_required(VERSION 3.31.0)
project(Freax LANGUAGES C CXX ASM)

# set cross comp settings
set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR aarch64)
set(CMAKE_C_COMPILER aarch64-suse-linux-gcc)
set(CMAKE_CXX_COMPILER aarch64-suse-linux-g++)
set(CMAKE_ASM_COMPILER aarch64-suse-linux-as)
set(CMAKE_OBJCOPY aarch64-suse-linux-objcopy)  # Added this line

# set compilation flags
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -ffreestanding -O2 -Wall -Wextra")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -ffreestanding -O2 -Wall -Wextra -fno-exceptions -fno-rtti")

# Linker script path
set(LINKER_SCRIPT "${CMAKE_SOURCE_DIR}/linker.ld")

# include dir
include_directories(${CMAKE_SOURCE_DIR}/include)

# gather source files
file(GLOB ASM_SOURCES "${CMAKE_SOURCE_DIR}/src/*.S")
file(GLOB CXX_SOURCES "${CMAKE_SOURCE_DIR}/src/*.cc")

add_executable(${PROJECT_NAME}.elf
    ${ASM_SOURCES}
    ${CXX_SOURCES}
)

# Force using C++ compiler for linking
set_target_properties(${PROJECT_NAME}.elf
    PROPERTIES
    LINK_FLAGS "-nostdlib -T${LINKER_SCRIPT}"
    LINKER_LANGUAGE CXX
)

add_custom_command(
    TARGET ${PROJECT_NAME}.elf POST_BUILD
    COMMAND ${CMAKE_OBJCOPY} -O binary ${PROJECT_NAME}.elf kernel8.img
)

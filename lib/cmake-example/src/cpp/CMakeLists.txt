cmake_minimum_required(VERSION 3.10)
project("cmake example for Rust" LANGUAGES CXX C)

set(CMAKE_CXX_STANDARD 17)

set(TARGET cmake-example)

# 必ずstaticライブラリにビルドすること
add_library(${TARGET}
    example.cc
)


target_include_directories(${TARGET}
    example.h
)

# 必ずこれが必要です
install (TARGETS ${TARGET} DESTINATION .)
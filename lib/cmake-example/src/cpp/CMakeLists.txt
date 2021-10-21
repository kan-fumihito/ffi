cmake_minimum_required(VERSION 3.10)
project("cmake-example" LANGUAGES CXX C)

set(CMAKE_CXX_STANDARD 17)

set(TARGET cmake-example)

find_package(Threads REQUIRED)

find_package(ament_cmake_ros REQUIRED)
find_package(ament_index_cpp REQUIRED)
find_package(builtin_interfaces REQUIRED)
find_package(libstatistics_collector REQUIRED)
find_package(rcl REQUIRED)
find_package(rcl_interfaces REQUIRED)
find_package(rcl_yaml_param_parser REQUIRED)
find_package(rcpputils REQUIRED)
find_package(rcutils REQUIRED)
find_package(rmw REQUIRED)
find_package(rosgraph_msgs REQUIRED)
find_package(rosidl_runtime_cpp REQUIRED)
find_package(rosidl_typesupport_c REQUIRED)
find_package(rosidl_typesupport_cpp REQUIRED)
find_package(statistics_msgs REQUIRED)
find_package(tracetools REQUIRED)
find_package(Python3 REQUIRED COMPONENTS Interpreter)

# 必ずstaticライブラリにビルドすること
add_library(${TARGET}
  STATIC
    example.cc
)

################  application related  ################
include_directories(${CMAKE_CURRENT_LIST_DIR})
include_directories(/opt/ros/galactic/include)
link_directories(/opt/ros/galactic/lib)

target_include_directories(${TARGET}
  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}
)

# specific order: dependents before dependencies
ament_target_dependencies(${TARGET}
  "ament_index_cpp"
  "libstatistics_collector"
  "rcl"
  "rcl_yaml_param_parser"
  "rcpputils"
  "rcutils"
  "builtin_interfaces"
  "rosgraph_msgs"
  "rosidl_typesupport_cpp"
  "rosidl_runtime_cpp"
  "statistics_msgs"
  "tracetools"
)

ament_package()

# 必ずこれが必要です
install (TARGETS ${TARGET} DESTINATION .)
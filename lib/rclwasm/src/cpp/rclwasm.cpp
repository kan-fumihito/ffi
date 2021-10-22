

#include <iostream>
#include <cstdint>
#include <cstring>
#include <string>
#include <map>

#include "rclwasm.hpp"
#include "std_msgs/msg/string.hpp"
#include "rcl/publisher.h"
#include "rcl/rcl.h"
#include "rosidl_typesupport_cpp/message_type_support.hpp"

const rosidl_message_type_support_t *string_ts =
    rosidl_typesupport_cpp::get_message_type_support_handle<std_msgs::msg::String>();

typedef struct wasm_node_t
{
    rcl_node_t *node;
    std::map<int, rcl_publisher_t *> publisher;
} wasm_node_t;

static rcl_context_t context;
static std::map<int, wasm_node_t *> node_pool;

extern "C"
{
    int foo(void)
    {
        std::cout << "Hello, C++" << std::endl;
        int ret = init_context();
        std::cout << "init_context() => " << ret << std::endl;
        return 42;
    }

    int bar(void)
    {
        std::cout << "Hello,bar C++" << std::endl;
        return 42;
    }

    int init_context(void)
    {
        // init options
        rcl_ret_t ret;
        rcl_init_options_t init_ops = rcl_get_zero_initialized_init_options();
        static rcl_allocator_t allocator = rcl_get_default_allocator();
        ret = rcl_init_options_init(&init_ops, allocator);
        if (ret != RCL_RET_OK)
        {
            std::cout << "init options status: " << ret << std::endl;
            return -1;
        }

        // init context
        context = rcl_get_zero_initialized_context();
        ret = rcl_init(0, nullptr, &init_ops, &context);
        if (ret != RCL_RET_OK)
        {
            std::cout << "init context status: " << ret << std::endl;
            return -1;
        }

        return 0;
    }
    int create_node(
        const char *node_name,
        const char *namespace_)
    {

        rcl_ret_t ret;
        static rcl_node_t node = rcl_get_zero_initialized_node();
        static rcl_node_options_t node_ops = rcl_node_get_default_options();

        ret = rcl_node_init(&node, node_name, namespace_, &context, &node_ops);
        if (ret != RCL_RET_OK)
        {
            std::cout << "init node status: " << ret << std::endl;
            return -1;
        }
        wasm_node_t *wasm_node = new wasm_node_t();
        wasm_node->node = &node;
        int id = 0;
        while (node_pool.find(id) != node_pool.end())
        {
            id++;
        }

        if (!rcl_node_is_valid(&node))
        {
            std::cout << "node is invalid." << std::endl;
            return -1;
        }

        node_pool.insert(std::make_pair(id, wasm_node));
        return id;
    }

    int create_publisher(
        int node_id,
        const char *topic_name)
    {
        rcl_ret_t ret;
        static rcl_publisher_t publisher = rcl_get_zero_initialized_publisher();
        rcl_publisher_options_t publisher_ops = rcl_publisher_get_default_options();

        wasm_node_t *wasm_node = node_pool.at(node_id);

        if (!rcl_node_is_valid(wasm_node->node))
        {
            std::cout << "node is invalid." << std::endl;
            return -1;
        }

        ret = rcl_publisher_init(&publisher, wasm_node->node, string_ts, topic_name, &publisher_ops);
        if (ret != RCL_RET_OK)
        {
            std::cout << "init publisher status: " << ret << std::endl;
            return -1;
        }

        int id = 0;
        while (wasm_node->publisher.find(id) != wasm_node->publisher.end())
        {
            id++;
        }
        wasm_node->publisher.insert(std::make_pair(id, &publisher));
        return id;
    }

    int publish(
        int node_id,
        int publisher_id,
        const char *data)
    {
        rcl_ret_t ret;
        std_msgs::msg::String msg;
        msg.data = std::string(data);

        wasm_node_t *wasm_node = node_pool.at(node_id);
        rcl_publisher_t *publisher = wasm_node->publisher.at(publisher_id);

        ret = rcl_publish(publisher, &msg, nullptr);
        if (ret == RCL_RET_OK)
        {
            std::cout << msg.data << std::endl;
            return 0;
        }
        else
        {
            std::cout << "ret: " << ret << std::endl;
            return -1;
        }
    }
}

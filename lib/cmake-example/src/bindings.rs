extern "C" {
    pub fn foo() -> i32;
    pub fn bar() -> i32;
    pub fn init_context() -> i32;
    pub fn create_node(node_name: *const str, _namespace: *const str) -> i32;
    pub fn create_publisher(node_id: i32, topic_name: *const str) -> i32;
    pub fn publish(node_id: i32, publisher_id: i32, data: *const str) -> i32;
}

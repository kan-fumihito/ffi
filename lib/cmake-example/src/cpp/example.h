
void foo(void);
int init_context(void);
int create_node(
    const char *node_name,
    const char *namespace_);
int create_publisher(
    int node_id,
    const char *topic_name);
int publish(
    int node_id,
    int publisher_id,
    const char *data);

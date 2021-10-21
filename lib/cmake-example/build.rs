use cmake;

fn main() {
    // CMakeLists.txtが存在するディレクトリを指定します
    // lib/cmake-exampleディレクトリからの相対位置となります
    let dst = cmake::build("src/cpp");
    println!("cargo:rustc-link-search=native={}", dst.display());

    // staticライブラリとして他に利用するライブラリはなし
    println!("cargo:rustc-link-lib=static=cmake-example");

    // C++ソースコードの場合は必ずこれを追加すること
    println!("cargo:rustc-link-lib=dylib=stdc++");

    // CMakeLists.txt内の記述とは別に、その他のライブラリは必要なものを全て記述する必要あり
    //println!("cargo:rustc-link-lib=dylib=");

    // pkg-configでヒットしないライブラリは以下のように直接パス指定が可能
    let ros_lib_dir = "/opt/ros/galactic/lib";
    println!("cargo:rustc-link-search={}", ros_lib_dir);
    println!("cargo:rustc-link-lib=dylib=rcl");
    println!("cargo:rustc-link-lib=dylib=rcl_action");
    println!("cargo:rustc-link-lib=dylib=rcl_interfaces__python");
    println!("cargo:rustc-link-lib=dylib=rcl_interfaces__rosidl_typesupport_c");
    println!("cargo:rustc-link-lib=dylib=rcl_interfaces__rosidl_typesupport_cpp");
    println!("cargo:rustc-link-lib=dylib=rcl_yaml_param_parser");
    println!("cargo:rustc-link-lib=dylib=rcpputils");
    println!("cargo:rustc-link-lib=dylib=rcutils");
    println!("cargo:rustc-link-lib=dylib=rmw");
    //println!("cargo:rustc-link-lib=dylib=rosgraph_msgs");
    println!("cargo:rustc-link-lib=dylib=rosidl_runtime_c");
    println!("cargo:rustc-link-lib=dylib=rosidl_typesupport_c");
    println!("cargo:rustc-link-lib=dylib=rosidl_typesupport_cpp");
    //println!("cargo:rustc-link-lib=dylib=statistics_msgs");
    println!("cargo:rustc-link-lib=dylib=std_msgs__rosidl_generator_c");
    println!("cargo:rustc-link-lib=dylib=std_msgs__rosidl_typesupport_c");
    println!("cargo:rustc-link-lib=dylib=std_msgs__rosidl_typesupport_cpp");
    
}
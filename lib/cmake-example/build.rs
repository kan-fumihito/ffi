use cmake::Config;

fn main() {
    // CMakeLists.txtが存在するディレクトリを指定します
    // lib/cmake-exampleディレクトリからの相対位置となります
    let dst = Config::new("./src/cpp")
                 .build();

    println!("cargo:rustc-link-search=native={}", dst.display());

    // staticライブラリとして他に利用するライブラリはなし
    println!("cargo:rustc-link-lib=static=cmake-example");

    // C++ソースコードの場合は必ずこれを追加すること
    println!("cargo:rustc-link-lib=dylib=stdc++");

    // CMakeLists.txt内の記述とは別に、その他のライブラリは必要なものを全て記述する必要あり
    //println!("cargo:rustc-link-lib=dylib=cmake-example");
    //println!("cargo:rustc-link-lib=dylib=GLESv2");
    //println!("cargo:rustc-link-lib=dylib=X11");

    // pkg-configでヒットしないライブラリは以下のように直接パス指定が可能
    //let soil_lib_dir = "/usr/lib";
    //println!("cargo:rustc-link-search={}", soil_lib_dir);
    //println!("cargo:rustc-link-lib=dylib=SOIL");
}

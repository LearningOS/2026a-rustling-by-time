//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // tests7:
    // 获取当前 Unix 时间戳
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 告诉 Cargo：
    // 在编译 Rust 代码时设置环境变量 TEST_FOO
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // tests8:
    // 告诉 rustc：编译时启用 cfg(feature = "pass")
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
pub fn write_stdlib() {
    let hpp_content = include_str!("std.hpp");
    let cpp_content = include_str!("std.cpp");

    std::fs::write("build/include/std.hpp", hpp_content).expect("Failed to write std.hpp");
    std::fs::write("build/src/std.cpp", cpp_content).expect("Failed to write std.cpp");
}

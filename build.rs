use cc::Build;
use bindgen::RustTarget;
use std::process::Command;

fn main() {
	Command::new("git")
		.args(&["submodule", "init"])
		.status()
		.expect("Unable to initialize git submodules");
	Command::new("git")
		.args(&["submodule", "update"])
		.status()
		.expect("Unable to update the submodule repositories");

	Build::new()
		.cpp(true)
		.include("src/yoga")
		// BASE_COMPILER_FLAGS: https://github.com/facebook/yoga/blob/master/tools/build_defs/oss/yoga_defs.bzl#L54
		.flag("-std=c++1y")
		.flag("-fno-omit-frame-pointer")
		.flag("-fexceptions")
		.flag("-fvisibility=hidden")
		.flag("-ffunction-sections")
		.flag("-fdata-sections")
		.flag("-Wall")
		.flag("-Werror")
		.flag("-O2")
		.flag("-std=c++11")
		.flag("-DYG_ENABLE_EVENTS")
		//
		.flag("-Wno-inconsistent-missing-override")
		// LIBRARY_COMPILER_FLAGS: https://github.com/facebook/yoga/blob/master/tools/build_defs/oss/yoga_defs.bzl#L66
		.flag("-fPIC")
		// C++ Files
		.file("src/yoga/yoga/Utils.cpp")
		.file("src/yoga/yoga/YGConfig.cpp")
		.file("src/yoga/yoga/YGEnums.cpp")
		.file("src/yoga/yoga/YGFloatOptional.cpp")
		.file("src/yoga/yoga/YGLayout.cpp")
		.file("src/yoga/yoga/YGNode.cpp")
		.file("src/yoga/yoga/YGNodePrint.cpp")
		.file("src/yoga/yoga/YGStyle.cpp")
		.file("src/yoga/yoga/Yoga.cpp")
		.compile("libyoga.a");

	// let bindings = bindgen::Builder::default()
	// 	.rust_target(RustTarget::Stable_1_40)
	// 	.no_convert_floats()
	// 	.enable_cxx_namespaces()
	// 	.whitelist_type("YG.*")
	// 	.whitelist_function("YG.*")
	// 	.whitelist_var("YG.*")
	// 	.layout_tests(false)
	// 	.rustfmt_bindings(true)
	// 	.rustified_enum("YG.*")
	// 	.header("src/yoga/yoga/Yoga.h")
	// 	.generate()
	// 	.expect("Unable to generate bindings");

	// bindings
	// 	.write_to_file("src/bindings.rs")
	// 	.expect("Unable to write bindings!");
}

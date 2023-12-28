extern crate bindgen;

use std::{
	env,
	fs::{self, create_dir_all, File},
	io::Write,
	path::{Path, PathBuf},
	process::{Command, Stdio},
};

struct CrossCompileInfo {
	pub sysroot: PathBuf,
	pub target_triple: String,
	pub deb_arch: String,
	pub toolchain_prefix: String,
}

fn main() {
	println!("cargo:rerun-if-changed=build.rs");

	let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
	let host_triple = env::var("HOST").unwrap();
	let target_triple = env::var("TARGET").unwrap();

	let cross_compile_info = if host_triple != target_triple {
		// We are cross-compiling
		let sysroot = PathBuf::from(env::var("CROSS_SYSROOT").expect("Cross-compiling requires CROSS_SYSROOT to be set"));
		let deb_arch = env::var("CROSS_DEB_ARCH").expect("Cross-compiling requires CROSS_DEB_ARCH to be set");
		let toolchain_prefix = env::var("CROSS_TOOLCHAIN_PREFIX").expect("Cross-compiling requires CROSS_TOOLCHAIN_PREFIX to be set");
		Some(CrossCompileInfo {
			sysroot,
			target_triple,
			deb_arch,
			toolchain_prefix,
		})
	} else {
		None
	};

	bazel_build(cross_compile_info.as_ref());

	let b_builder = bindgen::Builder::default()
		.clang_arg("-xc++")
		.clang_arg("-std=c++14")
		.generate_comments(true)
		.header("mediapipe/mediapipe/mediagraph.h")
		// Tell cargo to invalidate the built crate whenever any of the
		// included header files changed.
		.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
		.allowlist_function("mediagraph.*")
		.allowlist_type("mediagraph.*")
		.allowlist_var("mediagraph.*")
		.detect_include_paths(true)
		.generate_inline_functions(true);
	let bindings = b_builder.generate().expect("Unable to generate bindings");

	bindings.write_to_file(out_dir.join("bindings.rs")).expect("Couldn't write bindings!");
}

fn bazel_build(cross_compile_info: Option<&CrossCompileInfo>) {
	println!("cargo:rerun-if-changed=mediapipe/mediapipe");
	let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
	let mediapipe_src_dir = PathBuf::from("mediapipe");

	let mut bazel_command = Command::new("bazelisk");
	cmd_add_bazel_args(&mut bazel_command, &out_dir, &mediapipe_src_dir);
	bazel_command.arg("build").arg("--config=linux");
	// Disables the "convenience" symlinks
	bazel_command.arg("--symlink_prefix=/");
	// bazel_command.arg("--sandbox_debug");
	bazel_command.arg("--define").arg("MEDIAPIPE_DISABLE_GPU=1");

	if let Some(cc_info) = cross_compile_info {
		// We are cross-compiling
		set_cross_toolchain(&mut bazel_command, &mediapipe_src_dir, cc_info);
	}

	bazel_command.arg("mediapipe:mediagraph");
	let mut handle = bazel_command.stdout(Stdio::inherit()).spawn().expect("Failed to start mediapipe build");
	let exitcode = handle.wait().expect("Failed to build mediapipe");
	if !exitcode.success() {
		panic!("Failed to build mediapipe")
	}

	// Returns the bazel-bin output directory
	let mut bazel_bin_command = Command::new("bazelisk");
	cmd_add_bazel_args(&mut bazel_bin_command, &out_dir, &mediapipe_src_dir);
	bazel_bin_command.arg("info").arg("bazel-bin");
	if let Some(cc_info) = cross_compile_info {
		bazel_bin_command.arg(format!("--cpu={}", cc_info.deb_arch));
	}
	let bazel_build_fail_msg = "Failed to get bazel bin path";
	let output = bazel_bin_command.output().expect(bazel_build_fail_msg);
	if !output.status.success() {
		eprint!("{}", String::from_utf8(output.stderr).unwrap());
		panic!("{}", bazel_build_fail_msg)
	}
	let bazel_bin_str = String::from_utf8(output.stdout).expect(bazel_build_fail_msg).replace("\n", "").replace("\r", "");
	let bazel_bin_path = PathBuf::from(bazel_bin_str);
	let libmediagraph_folder_path = bazel_bin_path.join("mediapipe");
	// TODO: handling for other os-ses
	let libmediagraph_file_path = libmediagraph_folder_path.join("libmediagraph.so");
	// Tell cargo to look for shared libraries in the specified directory
	println!("cargo:rustc-link-search={}", libmediagraph_folder_path.display());
	// Tell cargo to tell rustc to link the mediagraph shared library.
	println!("cargo:rustc-link-lib=mediagraph");

	// Copy libmediagraph.so to main target directory
	match get_cargo_target_dir() {
		Ok(target_dir) => {
			println!("Found main cargo target dir: {}\nAdding libmediagraph.so...", target_dir.display());
			let target_file_path = target_dir.join("libmediagraph.so");
			if let Err(e) = fs::copy(&libmediagraph_file_path, &target_file_path) {
				eprintln!("Failed to copy to {}: {}", target_file_path.display(), e);
			}
			let target_file_path_examples = target_dir.join("examples").join("libmediagraph.so");
			if let Err(e) = fs::copy(&libmediagraph_file_path, &target_file_path_examples) {
				eprintln!("Failed to copy to {}: {}", target_file_path_examples.display(), e);
			}
		}
		Err(e) => eprintln!("Main cargo dir not found: {}\nYou might have to copy {} into the executable's folder", e, libmediagraph_file_path.display()),
	}
}

fn set_cross_toolchain(cmd: &mut Command, mediapipe_src_dir: &Path, cc_info: &CrossCompileInfo) {
	let toolchain_name = format!("cross_rs_docker_{arch_deb}", arch_deb = cc_info.deb_arch);

	let opencv_inc_path = format!("-I{}", cc_info.sysroot.join("include").join("opencv4").display());
	let mut compiler_args = vec!["-flto", "-O3", "-lstdc++", "-fPIC", &opencv_inc_path];

	if cc_info.target_triple.contains("armv7") {
		compiler_args.push("-march=armv7-a");
		compiler_args.push("-mfpu=neon-vfpv4");
		// Required for vectorization
		compiler_args.push("-funsafe-math-optimizations");
		// Enables required fp16
		compiler_args.push("-mfp16-format=ieee");
	}

	let compiler_args_str = bazel_format_args_array(&compiler_args);

	let toolchain_build = format!(
		r##"
package(default_visibility=["//visibility:public"])

cc_toolchain_suite(
    name="{toolchain_name}",
    toolchains={{
        "{arch_deb}": ":cross_toolchain_{arch_deb}",
    }},
)

filegroup(name="empty")
cc_toolchain(
    name="cross_toolchain_{arch_deb}",
    toolchain_identifier="cross_toolchain_{arch_deb}",
    toolchain_config=":cross_toolchain_config_{arch_deb}",
    all_files=":empty",
    compiler_files=":empty",
    dwp_files=":empty",
    linker_files=":empty",
    objcopy_files=":empty",
    strip_files=":empty",
    supports_param_files=0,
)

load(":cc_toolchain_config.bzl", "cc_toolchain_config_{arch_deb}")
cc_toolchain_config_{arch_deb}(name="cross_toolchain_config_{arch_deb}")
"##,
		toolchain_name = toolchain_name,
		arch_deb = cc_info.deb_arch
	);

	let toolchain_config = format!(
		r##"
load("@bazel_tools//tools/build_defs/cc:action_names.bzl", "ACTION_NAMES")
load(
    "@bazel_tools//tools/cpp:cc_toolchain_config_lib.bzl",
    "feature",
    "flag_group",
    "flag_set",
    "tool_path",
)

all_link_actions = [
    ACTION_NAMES.cpp_link_executable,
    ACTION_NAMES.cpp_link_dynamic_library,
    ACTION_NAMES.cpp_link_nodeps_dynamic_library,
]

all_compile_actions = [
    ACTION_NAMES.c_compile,
    ACTION_NAMES.cpp_compile,
]

def _impl(ctx):
    tool_paths = [
        tool_path(
            name = "gcc",
            path = "/usr/bin/{prefix}gcc",
        ),
        tool_path(
            name = "ld",
            path = "/usr/bin/{prefix}ld",
        ),
        tool_path(
            name = "ar",
            path = "/usr/bin/{prefix}ar",
        ),
        tool_path(
            name = "cpp",
            path = "/usr/bin/{prefix}cpp",
        ),
        tool_path(
            name = "gcov",
            path = "/usr/bin/{prefix}gcov",
        ),
        tool_path(
            name = "nm",
            path = "/usr/bin/{prefix}nm",
        ),
        tool_path(
            name = "objdump",
            path = "/usr/bin/{prefix}objdump",
        ),
        tool_path(
            name = "strip",
            path = "/usr/bin/{prefix}strip",
        ),
    ]

    features = [
        feature(
            name = "default_linker_flags",
            enabled = True,
            flag_sets = [
                flag_set(
                    actions = all_link_actions,
                    flag_groups = ([
                        flag_group(
                            flags = [
                                {compiler_args}
                            ],
                        ),
                    ]),
                ),
            ],
        ),
        feature(
            name = "default_compiler_flags",
            enabled = True,
            flag_sets = [
                flag_set(
                    actions = all_compile_actions,
                    flag_groups = ([
                        flag_group(
                            flags = [
                                {compiler_args}
                            ],
                        ),
                    ]),
                ),
            ],
        ),
    ]


    return cc_common.create_cc_toolchain_config_info(
        ctx = ctx,
        features = features,
        cxx_builtin_include_directories = [
            # This only relates to sandboxing. Just allow anything :P
            "/usr",
        ],
        toolchain_identifier = "cross_toolchain_{arch_deb}",
        host_system_name = "local",
        target_system_name = "local",
        target_cpu = "{arch_deb}",
        target_libc = "unknown",
        compiler = "gcc",
        abi_version = "unknown",
        abi_libc_version = "unknown",
        tool_paths = tool_paths,
    )

cc_toolchain_config_{arch_deb} = rule(
    implementation = _impl,
    attrs = {{}},
    provides = [CcToolchainConfigInfo],
)
"##,
		arch_deb = cc_info.deb_arch,
		prefix = cc_info.toolchain_prefix,
		compiler_args = compiler_args_str
	);

	let toolchain_base_path = format!("toolchain.generated/{target}", target = cc_info.target_triple);
	let toolchain_path = mediapipe_src_dir.join(&toolchain_base_path);
	create_dir_all(&toolchain_path).expect(&format!("Unable to crate directory: {}", toolchain_path.display()));
	write_file(&toolchain_path.join("BUILD"), &toolchain_build);
	write_file(&toolchain_path.join("cc_toolchain_config.bzl"), &toolchain_config);

	cmd.arg("--platforms=//mediapipe:linux_platform");
	cmd.arg(format!("--crosstool_top=//{}:{}", toolchain_base_path, toolchain_name));
	cmd.arg("--host_crosstool_top=@bazel_tools//tools/cpp:toolchain");
	cmd.arg(format!("--cpu={}", cc_info.deb_arch));
	// TODO enable -mfpu=neon for arm targets
}

fn cmd_add_bazel_args(cmd: &mut Command, out_dir: &Path, mediapipe_src_dir: &Path) {
	let bazel_output_base = out_dir.join("bazel");
	let bazel_user_root = out_dir.join("bazel-user");
	let bazelisk_home = out_dir.join("bazelisk");

	cmd.current_dir(mediapipe_src_dir);
	cmd.env("BAZELISK_HOME", bazelisk_home);
	cmd.arg(format!("--output_base={}", bazel_output_base.display()));
	cmd.arg(format!("--output_user_root={}", bazel_user_root.display()));
}

fn write_file(path: &Path, str: &str) {
	let mut file = match File::create(&path) {
		Err(e) => panic!("Couldn't create {}: {}", path.display(), e),
		Ok(file) => file,
	};
	if let Err(e) = file.write_all(str.as_bytes()) {
		panic!("Couldn't write to {}: {}", path.display(), e)
	}
}

fn bazel_format_args_array(strings: &[&str]) -> String {
	let mut result = String::new();
	for str in strings {
		result.push_str(&format!("\"{}\"", str));
		result.push_str(",\n");
	}
	result
}

// Used for debugging
fn _print_env() {
	let mut env_cmd = Command::new("env");
	let mut handle = env_cmd.stdout(Stdio::inherit()).spawn().expect("Failed to env");
	handle.wait().expect("Failed to env");
}

// Hacky way: https://github.com/rust-lang/cargo/issues/9661#issuecomment-1722358176
// To be changed when there is an official solution
/// Gets the output directory for final build artifacts
fn get_cargo_target_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
	let out_dir = PathBuf::from(env::var("OUT_DIR").expect("Expected OUT_DIR to be set in build script"));
	let profile = env::var("PROFILE").expect("Expected PROFILE to be set in build script");
	let mut target_dir = None;
	let mut sub_path = out_dir.as_path();
	while let Some(parent) = sub_path.parent() {
		if parent.ends_with(&profile) {
			target_dir = Some(parent);
			break;
		}
		sub_path = parent;
	}
	let target_dir = target_dir.ok_or("Target dir not found")?;
	Ok(target_dir.to_path_buf())
}

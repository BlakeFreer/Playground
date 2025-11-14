Import("env")
import subprocess
import os
import glob

build_dir = env.subst("$BUILD_DIR")
generated_src_dir = os.path.join(build_dir, "protoc", "generated-src")
generated_build_dir = os.path.join(build_dir, "protoc", "generated-build")

os.makedirs(generated_src_dir, exist_ok=True)
os.makedirs(generated_build_dir, exist_ok=True)

grpc_path = os.path.expanduser(env.GetProjectOption("custom_grpc_path"))

proto_src = glob.glob("proto/*.proto")
protoc = os.path.join(grpc_path, "bin", "protoc")
cpp_grpc = os.path.join(grpc_path, "bin", "grpc_cpp_plugin")

subprocess.run(
    [
        protoc,
        f"--cpp_out={generated_src_dir}",
        f"--grpc_out={generated_src_dir}",
        f"--plugin=protoc-gen-grpc={cpp_grpc}",
        *proto_src,
    ],
    check=True,
)

env.BuildSources(generated_build_dir, generated_src_dir)
env.Append(CPPPATH=[generated_src_dir])

pkg_path = os.path.join(grpc_path, "lib", "pkgconfig")
env.ParseConfig(f"pkg-config grpc++ --cflags --libs --with-path={pkg_path}")
env.ParseConfig(f"pkg-config protobuf --cflags --libs --with-path={pkg_path}")
env.ParseConfig(f"pkg-config upb --cflags --libs --with-path={pkg_path}")
env.ParseConfig(f"pkg-config re2 --cflags --libs --with-path={pkg_path}")
env.ParseConfig(f"pkg-config libcares --cflags --libs --with-path={pkg_path}")

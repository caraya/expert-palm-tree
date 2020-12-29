workspace(
    name = "bazel_kitchensink",
    managed_directories = {"@npm": ["node_modules"]},
)

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# rules_go
# These have to go first, otherwise Bazel will throw an error
http_archive(
    name = "io_bazel_rules_go",
    sha256 = "7904dbecbaffd068651916dce77ff3437679f9d20e1a7956bff43826e7645fcc",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/rules_go/releases/download/v0.25.1/rules_go-v0.25.1.tar.gz",
        "https://github.com/bazelbuild/rules_go/releases/download/v0.25.1/rules_go-v0.25.1.tar.gz",
    ],
)

load("@io_bazel_rules_go//go:deps.bzl", "go_register_toolchains", "go_rules_dependencies")

go_rules_dependencies()

go_register_toolchains(version = "1.15.6")

# Loads rules_nodejs
http_archive(
    name = "build_bazel_rules_nodejs",
    sha256 = "b3521b29c7cb0c47a1a735cce7e7e811a4f80d8e3720cf3a1b624533e4bb7cb6",
    urls = ["https://github.com/bazelbuild/rules_nodejs/releases/download/2.3.2/rules_nodejs-2.3.2.tar.gz"],
)

load("@build_bazel_rules_nodejs//:index.bzl", "yarn_install")

# Loads rules_postcss
http_archive(
    name = "build_bazel_rules_postcss",
    sha256 = "3f0c754f97e3940ea90f4d6408bfb2aefb3850e7941572b22b1b88579c428ff9",
    strip_prefix = "rules_postcss-0.5.0",
    # Make sure to check for the latest version when you install
    url = "https://github.com/bazelbuild/rules_postcss/archive/0.5.0.tar.gz",
)

# Loads rules_sass
http_archive(
    name = "io_bazel_rules_sass",
    sha256 = "9dcfba04e4af896626f4760d866f895ea4291bc30bf7287887cefcf4707b6a62",
    strip_prefix = "rules_sass-1.26.3",
    # Make sure to check for the latest version when you install
    url = "https://github.com/bazelbuild/rules_sass/archive/1.26.3.zip",
)

# Fetch required transitive dependencies. This is an optional step
load("@io_bazel_rules_sass//:package.bzl", "rules_sass_dependencies")

rules_sass_dependencies()

# Setup repositories which are needed for the Sass rules.
load("@io_bazel_rules_sass//:defs.bzl", "sass_repositories")

sass_repositories()

# Rust and WASM
http_archive(
    name = "io_bazel_rules_rust",
    sha256 = "0e2e633bf0f7f25392ffb477d677c88eb34fe70ffae05e3ad92fdd9f8d6579db",
    strip_prefix = "rules_rust-bc0578798f50d018ca4278ad5610598c400992c9",
    urls = [
        # Master branch as of 2020-12-05
        "https://github.com/bazelbuild/rules_rust/archive/bc0578798f50d018ca4278ad5610598c400992c9.tar.gz",
    ],
)

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")

rust_repositories()

# NPM/Yarn
yarn_install(
    name = "npm",
    package_json = "//:package.json",
    yarn_lock = "//:yarn.lock",
)

# Protractor
load("@npm//@bazel/protractor:package.bzl", "npm_bazel_protractor_dependencies")

npm_bazel_protractor_dependencies()

# Setup the rules_webtesting toolchain
load("@io_bazel_rules_webtesting//web:repositories.bzl", "web_test_repositories")

web_test_repositories()

load("@io_bazel_rules_webtesting//web/versioned:browsers-0.3.2.bzl", "browser_repositories")

browser_repositories(
    chromium = True,
    firefox = True,
)

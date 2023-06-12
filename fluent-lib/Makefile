NOW_DATE=$(shell date +"%Y-%m-%dT%H:%M:%S")
COMMIT_HASH=$(shell git rev-parse --short HEAD)
BUILD_NUM=$(shell cat ./build/BUILD_NUM)
NEXT_BUILD_NUM=$(shell echo $$(($(BUILD_NUM)+1)))
VERSION = "0.1.3"

VERSION_RS =  "pub const VERSION: &'static str = \"$(VERSION)\";"
COMMIT_HASH_RS = "pub const COMMIT_HASH: &'static str = \"$(COMMIT_HASH)\";"
BUILD_NUM_RS = "pub const BUILD_NUM: i64 = $(BUILD_NUM);"
NOW_DATE_RS = "pub const BUILD_AT:  &'static str = \"$(NOW_DATE)\";"

version:
	echo $(VERSION_RS) > ./src/common/version.rs
	echo  $(COMMIT_HASH_RS) >> ./src/common/version.rs
	echo  $(BUILD_NUM_RS) >> ./src/common/version.rs
	echo  $(NOW_DATE_RS) >> ./src/common/version.rs

build-dart:
	flutter_rust_bridge_codegen --rust-input ./src/app/api.rs \
	--dart-output ./build/dart/api_bridge.dart \
	--c-output ./build/lib/libfluent_lib.h \
	--rust-output ./src/api_bridge.rs

test:
	cargo test

# ~/Library/Containers/com.fluent.agent.fluent/Data
mv-lib:
	cp build/dart/api_bridge.dart ../lib/
	cp build/lib/fluent_lib-debug.dylib ../macos/lib/

build-rs:
	cargo build
	cp target/debug/libfluent_lib.dylib build/lib/fluent_lib-debug.dylib
	echo $(NEXT_BUILD_NUM) > ./build/BUILD_NUM

build-rs-release:
	# macOS arm64
	cargo clean
	cargo build --release
	cp target/release/libfluent_lib.dylib build/lib/fluent_lib-apple-arm64-$(VERSION)-$(BUILD_NUM).dylib
	# macOS x86_64
	cargo clean
	cargo build --release --target x86_64-apple-darwin
	cp target/x86_64-apple-darwin/release/libfluent_lib.dylib build/lib/fluent_lib-apple-x86-64-$(VERSION)-$(BUILD_NUM).dylib
	# windows x86_64
	cargo clean
	cargo build --release --target=x86_64-pc-windows-gnu
	cp target/x86_64-pc-windows-gnu/release/fluent_lib.dll build/lib/fluent_lib-windows-x86-64-$(VERSION)-$(BUILD_NUM).dylib
	echo $(NEXT_BUILD_NUM) > ./build/BUILD_NUM

.PHONY: build
build: version build-dart  build-rs mv-lib

build-release-apple: version build-dart version build-rs-release








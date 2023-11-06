
run: build
  ./target/bootstrap/yfreight build
  ./target/bootstrap/yfreight help

build_lib:
    rustc src/lib.rs --edition 2021 --crate-type=lib --crate-name=yfreight \
    --out-dir=target/bootstrap

build_bin:
    rustc src/main.rs --edition 2021 --crate-type=bin --crate-name=yfreight \
    --out-dir=target/bootstrap  -L target/bootstrap --extern yfreight

build:
  rm -dr target
  mkdir -p target/bootstrap
  # Build crate dependencies
  just build_lib
  # Create the executable
  just build_bin


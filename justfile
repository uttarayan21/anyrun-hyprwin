target_dir := env_var_or_default('CARGO_TARGET_DIR', 'target')

build:
    cargo build --release
install:
    cp {{target_dir}}/release/libhyprwin.so ~/.config/anyrun/plugins/

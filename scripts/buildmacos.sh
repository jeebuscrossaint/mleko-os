cd ..
cargo rustc -- -C link-args="-e __start -static -nostartfiles"

// !!! THIS IS A GENERATED FILE !!!
// ANY MANUAL EDITS MAY BE OVERWRITTEN AT ANY TIME
// Files autogenerated with cargo build (build/wasitests.rs).

#[test]
fn test_fd_sync() {
    assert_wasi_output!(
        "../../wasitests/fd_sync.wasm",
        "fd_sync",
        vec![],
        vec![(
            ".".to_string(),
            ::std::path::PathBuf::from("wasitests/test_fs/temp")
        ),],
        vec![],
        "../../wasitests/fd_sync.out"
    );
}
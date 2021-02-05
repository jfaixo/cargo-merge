use cargo_merge::merge::{Merge, detect_package_root};
use cargo_merge::opts::Opts;
use std::fs;
use std::path::PathBuf;

static mut BASE_DIR: Option<PathBuf> = None;

unsafe fn reset_base_dir() {
    if BASE_DIR.is_none() {
        BASE_DIR = Some(std::env::current_dir().unwrap())
    }
    else {
        let base_path = BASE_DIR.clone().unwrap();
        std::env::set_current_dir(base_path).unwrap();
    }
}

#[test]
fn simple_binary() {
    unsafe { reset_base_dir(); }
    let test_path = "tests_data/simple_binary";

    // Change current directory to the test directory
    let test_path = detect_package_root().join(test_path);
    std::env::set_current_dir(&test_path).unwrap();

    let opts = Opts { remove_error_output: false, debug: false };
    let merge = Merge::new(opts);
    merge.run();

    let expected = fs::read_to_string(test_path.join("expected_output.rs")).unwrap();
    let result = fs::read_to_string(test_path.join("target/merge/merged.rs")).unwrap();

    assert_eq!(expected, result);
}

#[test]
fn simple_binary_silenced() {
    unsafe { reset_base_dir(); }
    let test_path = "tests_data/simple_binary";

    // Change current directory to the test directory
    let test_path = detect_package_root().join(test_path);
    std::env::set_current_dir(&test_path).unwrap();

    let opts = Opts { remove_error_output: true, debug: false };
    let merge = Merge::new(opts);
    merge.run();

    let expected = fs::read_to_string(test_path.join("expected_output_silenced.rs")).unwrap();
    let result = fs::read_to_string(test_path.join("target/merge/merged.rs")).unwrap();

    assert_eq!(expected, result);
}

#[test]
fn simple_lib() {
    unsafe { reset_base_dir(); }
    let test_path = "tests_data/simple_lib";

    // Change current directory to the test directory
    let test_path = detect_package_root().join(test_path);
    std::env::set_current_dir(&test_path).unwrap();

    let opts = Opts { remove_error_output: false, debug: false };
    let merge = Merge::new(opts);
    merge.run();

    let expected = fs::read_to_string(test_path.join("expected_output.rs")).unwrap();
    let result = fs::read_to_string(test_path.join("target/merge/merged.rs")).unwrap();

    assert_eq!(expected, result);
}

#[test]
fn lib_and_bin() {
    unsafe { reset_base_dir(); }
    let test_path = "tests_data/lib_and_bin";

    // Change current directory to the test directory
    let test_path = detect_package_root().join(test_path);
    std::env::set_current_dir(&test_path).unwrap();

    let opts = Opts { remove_error_output: false, debug: false };
    let merge = Merge::new(opts);
    merge.run();

    let expected = fs::read_to_string(test_path.join("expected_output.rs")).unwrap();
    let result = fs::read_to_string(test_path.join("target/merge/merged.rs")).unwrap();

    assert_eq!(expected, result);
}

#[test]
fn import_external_lib() {
    unsafe { reset_base_dir(); }
    let test_path = "tests_data/import_external_lib";

    // Change current directory to the test directory
    let test_path = detect_package_root().join(test_path);
    std::env::set_current_dir(&test_path).unwrap();

    let opts = Opts { remove_error_output: false, debug: false };
    let merge = Merge::new(opts);
    merge.run();

    let expected = fs::read_to_string(test_path.join("expected_output.rs")).unwrap();
    let result = fs::read_to_string(test_path.join("target/merge/merged.rs")).unwrap();

    assert_eq!(expected, result);
}

#[test]
fn nested_crates() {
    unsafe { reset_base_dir(); }
    let test_path = "tests_data/nested_crates";

    // Change current directory to the test directory
    let test_path = detect_package_root().join(test_path);
    std::env::set_current_dir(&test_path).unwrap();

    let opts = Opts { remove_error_output: false, debug: false };
    let merge = Merge::new(opts);
    merge.run();

    let expected = fs::read_to_string(test_path.join("expected_output.rs")).unwrap();
    let result = fs::read_to_string(test_path.join("target/merge/merged.rs")).unwrap();

    assert_eq!(expected, result);
}
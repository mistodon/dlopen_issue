#[cfg(not(target_os = "macos"))]
compile_error!("This only runs on macOS!");

/// Just runs `cargo build` in the `dylib` folder.
fn recompile_dylib() {
    let output = std::process::Command::new("cargo")
        .arg("build")
        .current_dir("dylib")
        .output()
        .unwrap();

    assert!(output.status.success(), "Compilation failed: {:#?}", output);
}

/// Reloads a dylib.
///
/// Drops first to `dlclose`.
/// Then loads to get new version.
pub fn reload(lib: &mut Option<libloading::Library>) {
    const DYLIB_PATH: &'static str = "./dylib/target/debug/libdylib.dylib";

    *lib = None; // Drop the old one first.
    *lib = Some(libloading::Library::new(&DYLIB_PATH).unwrap())
}

/// Calls the `get_version` method in the dynamically linked `dylib` crate.
pub fn get_version(lib: &Option<libloading::Library>) -> &'static str {
    let lib = lib.as_ref().unwrap();

    unsafe {
        let symbol: libloading::Symbol<extern "C" fn() -> &'static str> =
            lib.get(b"get_version").unwrap();
        symbol()
    }
}

fn main() {
    let variant = std::env::args().skip(1).next();
    let working = match variant.as_ref().map(String::as_ref) {
        Some("working") => true,
        Some("broken") => false,
        _ => panic!("Please pass either `working` or `broken` as an argument."),
    };

    let mut lib = None;

    // Write and compile dylib - version 1
    {
        let working_version_1_source = r#"
#[no_mangle]
pub extern fn get_version() -> &'static str {
    "One"
}"#;
        let broken_version_1_source = r#"
#[no_mangle]
pub extern fn get_version() -> &'static str {
    print!(""); // This is the only change! Just this empty `print!`
    "One"
}"#;

        let source = if working {
            working_version_1_source
        } else {
            broken_version_1_source
        };

        std::fs::write("./dylib/src/lib.rs", source).unwrap();
        recompile_dylib();
    }

    // Load the dylib
    reload(&mut lib);

    // Call a function
    {
        let message = get_version(&lib);
        println!("Old library version: `{}`", message);
        assert_eq!(message, "One");
    }

    // Write and compile dylib - version 2
    {
        let version_2_source = r#"
    #[no_mangle]
    pub extern fn get_version() -> &'static str {
        "Two"
    }"#;
        std::fs::write("./dylib/src/lib.rs", version_2_source).unwrap();
        recompile_dylib();
    }

    // Reload the changed dylib
    reload(&mut lib);

    // Call a function
    {
        let message = get_version(&lib);
        println!("New library version: `{}`", message);
        assert_eq!(message, "Two", "\n\nLooks like the new version of the dylib wasn't loaded and it's still calling the old version!");
    }
}

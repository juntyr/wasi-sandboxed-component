use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
};

use ::{
    virtual_wasi_cli as _, virtual_wasi_clocks as _, virtual_wasi_filesystem as _,
    virtual_wasi_io as _, virtual_wasi_random as _,
};

fn main() -> io::Result<()> {
    // Check for `clippy` and skip compilation in that case
    //  since `clippy` pollutes the `RUSTFLAGS` between rebuilds
    let is_clippy = std::env::var_os("RUSTC_WRAPPER")
        .or_else(|| std::env::var_os("RUSTC_WORKSPACE_WRAPPER"))
        .map_or(false, |wrapper| {
            Path::new(&wrapper).ends_with("clippy-driver")
        });

    let scratch_dir = scratch::path(concat!(
        env!("CARGO_PKG_NAME"),
        "-",
        env!("CARGO_PKG_VERSION"),
    ));
    let target_dir = scratch_dir.join("target");

    let components_path = scratch_dir.join("components.rs");

    eprintln!("scratch_dir={scratch_dir:?}");
    eprintln!("target_dir={target_dir:?}");
    eprintln!("components_path={components_path:?}");

    eprintln!("creating {target_dir:?}");
    fs::create_dir_all(&target_dir)?;

    let mut components = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&components_path)?;

    let mut all_component_const_names = Vec::new();

    for (crate_name, component_name) in [
        ("virtual-wasi-cli", "virtual-wasi:cli"),
        ("virtual-wasi-clocks", "virtual-wasi:clocks"),
        ("virtual-wasi-filesystem", "virtual-wasi:filesystem"),
        ("virtual-wasi-io", "virtual-wasi:io"),
        ("virtual-wasi-random", "virtual-wasi:random"),
    ] {
        let const_name = crate_name.to_uppercase().replace('-', "_");

        let wasm = if is_clippy {
            PathBuf::from("/dev/null")
        } else {
            let wasm = build_wasm_module(&target_dir, crate_name)?;
            add_change_dependencies(&wasm)?;
            create_new_component(&wasm)?
        };

        writeln!(
            &mut components,
            "pub const {const_name}: &[u8] = include_bytes!({wasm:?});"
        )?;

        all_component_const_names.push((component_name, const_name));
    }

    writeln!(
        &mut components,
        "pub const ALL_COMPONENTS: &[(&str, &[u8])] = &[{}];",
        all_component_const_names
            .into_iter()
            .map(|(component_name, const_name)| format!("({component_name:?}, {const_name})"))
            .collect::<Vec<_>>()
            .join(", "),
    )?;

    components.flush()?;

    println!("cargo::rustc-env=COMPONENTS={}", components_path.display());

    Ok(())
}

fn configure_cargo_cmd() -> io::Result<Command> {
    let cargo =
        PathBuf::from(std::env::var_os("CARGO").ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "missing env variable `CARGO`")
        })?);

    eprintln!("cargo={cargo:?}");

    // Special-case for compilation inside Pyodide, where we need to explicitly
    //  circumvent the cross-compilation wrapper
    let mut cmd = if cargo.ends_with("pywasmcross.py") {
        Command::new("cargo")
    } else {
        Command::new(cargo)
    };

    cmd.env_remove("CARGO_ENCODED_RUSTFLAGS");

    // we don't need nightly Rust features but need to compile std with immediate
    // panic abort instead of compiling with nightly, we fake it and forbid the
    // unstable_features lint
    if !Path::new(cmd.get_program())
        .components()
        .any(|c| c.as_os_str().as_encoded_bytes().starts_with(b"nightly-"))
    {
        cmd.env("RUSTC_BOOTSTRAP", "1");
    }

    Ok(cmd)
}

fn build_wasm_module(target_dir: &Path, crate_name: &str) -> io::Result<PathBuf> {
    let mut cmd = configure_cargo_cmd()?;
    cmd.arg("rustc")
        .arg("--crate-type=cdylib")
        .arg("-Z")
        .arg("build-std=std,panic_abort")
        .arg("-Z")
        .arg("build-std-features=panic_immediate_abort")
        .arg("--release")
        .arg("--target=wasm32-unknown-unknown")
        .arg("--package")
        .arg(crate_name)
        .env("RUSTFLAGS", "-C panic=abort -C strip=symbols")
        .env("CARGO_TARGET_DIR", target_dir);

    eprintln!("executing {cmd:?}");

    let status = cmd.status()?;
    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("cargo exited with code {status}"),
        ));
    }

    Ok(target_dir
        .join("wasm32-unknown-unknown")
        .join("release")
        .join(crate_name.replace('-', "_"))
        .with_extension("wasm"))
}

fn add_change_dependencies(wasm: &Path) -> io::Result<()> {
    let dep_file = wasm.with_extension("d");

    eprintln!("reading {dep_file:?}");
    let deps = fs::read_to_string(dep_file)?;

    let Some((_, deps)) = deps.split_once(':') else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid deps file format",
        ));
    };

    for dep in deps.split_whitespace() {
        println!("cargo::rerun-if-changed={dep}");
    }

    Ok(())
}

fn create_new_component(wasm: &Path) -> io::Result<PathBuf> {
    let wasm_component = wasm.with_extension("component.wasm");

    eprintln!("reading from {wasm:?}");
    let wasm = fs::read(wasm)?;

    let mut encoder = wit_component::ComponentEncoder::default()
        .module(&wasm)
        .map_err(|err| {
            io::Error::new(
                io::ErrorKind::Other,
                // FIXME: better error reporting in the build script
                format!("wit_component::ComponentEncoder::module failed: {err:#}"),
            )
        })?;

    let wasm = encoder.encode().map_err(|err| {
        io::Error::new(
            io::ErrorKind::Other,
            // FIXME: better error reporting in the build script
            format!("wit_component::ComponentEncoder::encode failed: {err:#}"),
        )
    })?;

    eprintln!("writing to {wasm_component:?}");
    fs::write(&wasm_component, wasm)?;

    Ok(wasm_component)
}

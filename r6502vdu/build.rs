fn main() {
    #[cfg(target_os = "macos")]
    macos::set_lib_path();

    #[cfg(target_os = "windows")]
    windows::copy_sdl3();
}

#[cfg(target_os = "macos")]
mod macos {
    use std::path::Path;
    use std::process::Command;

    pub fn set_lib_path() {
        let output = Command::new("brew").arg("--prefix").output().unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap();
        let lib_path = Path::new(stdout.trim()).join("lib");
        println!(
            "cargo:rustc-env=LIBRARY_PATH={lib_path}",
            lib_path = lib_path.display()
        );
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use std::env::var;
    use std::fs::copy;
    use std::io::ErrorKind;
    use std::path::{Path, PathBuf};

    pub fn copy_sdl3() {
        let workspace_dir = get_workspace_dir();
        let output_dir = get_output_dir(&workspace_dir);
        for file_name in ["SDL3_ttf.dll", "SDL3_ttf.lib", "SDL3.dll", "SDL3.lib"] {
            let from = workspace_dir.join(file_name);
            let to = output_dir.join(file_name);
            match copy(&from, to) {
                Ok(_) => {}
                Err(e) if e.kind() == ErrorKind::NotFound => {
                    panic!("file {from} not found", from = from.display())
                }
                Err(e) => panic!("{e:?}"),
            }
        }
    }

    fn get_workspace_dir() -> PathBuf {
        let s = var("CARGO_MANIFEST_DIR").unwrap();
        let manifest_dir = Path::new(&s);
        let workspace_dir = manifest_dir.parent().unwrap();
        workspace_dir.to_path_buf()
    }

    fn get_output_dir(workspace_dir: &Path) -> PathBuf {
        let profile = var("PROFILE").unwrap();
        workspace_dir.join("target").join(&profile)
    }
}

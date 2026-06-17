use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

fn main() {
    #[cfg(target_os = "linux")]
    {
        let project_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

        let exec_path = project_dir.join("target/debug/BIN_OCR");
        let icon_path = project_dir.join("BIN_app.ico");

        let _ = fs::remove_file(project_dir.join("BIN_OCR.desktop"));

        if let Some(home_dir) = env::var_os("HOME").map(PathBuf::from) {
            let apps_dir = home_dir.join(".local/share/applications");
            let _ = fs::remove_file(apps_dir.join("BIN_OCR.desktop"));
            let _ = fs::remove_file(apps_dir.join("BIN_OCR.desktop"));
        }

        let desktop_content = format!(
            "[Desktop Entry]\n\
            Type=Application\n\
            Version=1.0\n\
            Name=Client BIN_OCR\n\
            Comment=Application Client EPITA\n\
            Exec={}\n\
            Icon={}\n\
            Terminal=false\n\
            Categories=Utility;Development;\n",
            exec_path.display(),
            icon_path.display()
        );

        if let Some(home_dir) = env::var_os("HOME").map(PathBuf::from) {
            let apps_dir = home_dir.join(".local/share/applications");

            let _ = fs::create_dir_all(&apps_dir);

            let desktop_file_path = apps_dir.join("BIN_OCR.desktop");

            // Écrire le fichier .desktop
            if let Ok(mut file) = File::create(&desktop_file_path) {
                let _ = file.write_all(desktop_content.as_bytes());

                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Ok(metadata) = fs::metadata(&desktop_file_path) {
                        let mut perms = metadata.permissions();
                        perms.set_mode(0o755); // chmod +x
                        let _ = fs::set_permissions(&desktop_file_path, perms);
                    }
                }
            }
        }
        let local_desktop_path = project_dir.join("../BIN_OCR.desktop");
        if let Ok(mut local_file) = File::create(&local_desktop_path) {
            let _ = local_file.write_all(desktop_content.as_bytes());
        }
    }

    #[cfg(target_os = "windows")]
    {
        let project_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let exec_path = project_dir.join("target\\debug\\client.exe");
        let icon_path = project_dir.join("BIN_app.ico");
        let _ = fs::remove_file(project_dir.join("BIN_OCR.desktop"));

        if let Some(home_dir) = env::var_os("HOME").map(PathBuf::from) {
            let apps_dir = home_dir.join(".local/share/applications");
            let _ = fs::remove_file(apps_dir.join("BIN_OCR.desktop"));
            let _ = fs::remove_file(apps_dir.join("BIN_OCR.desktop"));
        }
        let desktop_content = format!(
            "[Desktop Entry]\n\
            Type=Application\n\
            Version=1.0\n\
            Name=Client BIN_OCR\n\
            Comment=Application Client EPITA\n\
            Exec={}\n\
            Icon={}\n\
            Terminal=false\n\
            Categories=Utility;Development;\n",
            exec_path.display().to_string().replace("\\", "/"),
            icon_path.display().to_string().replace("\\", "/")
        );

        let local_desktop_path = project_dir.join("BIN_OCR.desktop");
        if let Ok(mut local_file) = File::create(&local_desktop_path) {
            let _ = local_file.write_all(desktop_content.as_bytes());
        }

        let mut res = winres::WindowsResource::new();
        res.set_icon("BIN_app.ico");
        let _ = res.compile();
    }
}

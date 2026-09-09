fn main() {
    // tauri-build already embeds the app manifest (with Common-Controls v6)
    // into BIN targets. TEST targets get no manifest, so on windows-gnu any
    // comctl32/user32 import (TaskDialogIndirect etc.) fails at process start
    // with STATUS_ENTRYPOINT_NOT_FOUND. Compile the same manifest with
    // windres and link it into every test binary to fix `cargo test`.
    #[cfg(all(target_os = "windows", target_env = "gnu"))]
    {
        let out = std::env::var("OUT_DIR").unwrap();
        let rc = std::path::Path::new(&out).join("luci-tests.manifest.rc");
        std::fs::write(
            &rc,
            "1 24 \"luci-tests.manifest\"\r\n",
        )
        .expect("write test manifest rc");
        std::fs::write(
            std::path::Path::new(&out).join("luci-tests.manifest"),
            "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n\
             <assembly xmlns=\"urn:schemas-microsoft-com:asm.v1\" manifestVersion=\"1.0\">\n\
             <dependency>\n\
             <dependentAssembly>\n\
             <assemblyIdentity type=\"win32\" name=\"Microsoft.Windows.Common-Controls\"\n\
             version=\"6.0.0.0\" processorArchitecture=\"*\" publicKeyToken=\"6595b64144ccf1df\" language=\"*\"/>\n\
             </dependentAssembly>\n\
             </dependency>\n\
             </assembly>\n",
        )
        .expect("write test manifest");
        // windres ships with the msys2/mingw64 toolchain used by windows-gnu.
        for windres in ["windres", "x86_64-w64-mingw32-windres"] {
            let ok = std::process::Command::new(windres)
                .args([
                    "-O",
                    "coff",
                    &std::path::Path::new(&rc).to_string_lossy(),
                    &std::path::Path::new(&out).join("luci-tests.manifest.o").to_string_lossy(),
                ])
                .output();
            if let Ok(o) = ok {
                if o.status.success() {
                    println!("cargo:rustc-link-arg-tests=-Wl,{}", 
                        std::path::Path::new(&out).join("luci-tests.manifest.o").to_string_lossy());
                    break;
                }
            }
        }
    }
    tauri_build::build()
}

//! Windows only: embed the app icon and version info (VERSIONINFO) into `fisiefeitos.exe`.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `EFFECTCRAFT_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/fisilab/fisiefeitos.ico");
    println!("cargo:rerun-if-env-changed=EFFECTCRAFT_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/fisilab/fisiefeitos.ico")
        .set("ProductName", "FisiEfeitos")
        .set("FileDescription", "FisiEfeitos — Animação gráfica e composição (Suíte FisiLab)")
        .set("CompanyName", "FisiLab — LARFIC/UFRGS")
        .set("LegalCopyright", "Baseado no EffectCraft (© ArtCraft Team e contribuidores). Modificações © 2026 FisiLab — LARFIC/UFRGS. MIT OR Apache-2.0.")
        .set("OriginalFilename", "fisiefeitos.exe")
        .set("InternalName", "fisiefeitos");
    if let Err(e) = res.compile() {
        if std::env::var_os("EFFECTCRAFT_REQUIRE_WINRES").is_some() {
            panic!("embedding Windows resources failed: {e}");
        }
        println!("cargo:warning=fisiefeitos.exe built without icon/version resources: {e}");
    }
}

fn main() {
    if cfg!(target_os = "windows") {
        // Embed windows app icon.
        let mut res = winresource::WindowsResource::new();
        res.set_icon("ui/images/window/app_icon.ico")
            .set("Olorin", "olorin.exe")
            .set_version_info(winresource::VersionInfo::PRODUCTVERSION, 0x0001000000000000);
        let _ = res.compile();
    }
}

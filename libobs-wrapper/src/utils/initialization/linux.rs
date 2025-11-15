//TODO
pub(crate) fn load_debug_privilege() {
    /*

    #if !defined(_WIN32) && !defined(__APPLE__)
        if (QApplication::platformName() == "xcb") {
    #if QT_VERSION >= QT_VERSION_CHECK(6, 5, 0)
            auto native = qGuiApp->nativeInterface<QNativeInterface::QX11Application>();

            obs_set_nix_platform_display(native->display());
    #endif

            obs_set_nix_platform(OBS_NIX_PLATFORM_X11_EGL);

            blog(LOG_INFO, "Using EGL/X11");
        }

    #ifdef ENABLE_WAYLAND
        if (QApplication::platformName().contains("wayland")) {
    #if QT_VERSION >= QT_VERSION_CHECK(6, 5, 0)
            auto native = qGuiApp->nativeInterface<QNativeInterface::QWaylandApplication>();

            obs_set_nix_platform_display(native->display());
    #endif

            obs_set_nix_platform(OBS_NIX_PLATFORM_WAYLAND);
            setAttribute(Qt::AA_DontCreateNativeWidgetSiblings);

            blog(LOG_INFO, "Platform: Wayland");
        }
    #endif */
}

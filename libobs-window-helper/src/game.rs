/// List is taken from <https://github.com/obsproject/obs-studio/blob/ce6f9a4742b40b04e68ef759e0698bd5eac4360e/plugins/win-capture/game-capture.c#L1040>
const BLACKLISTED_EXE: &'static [&'static str] = &[
    "explorer",
    "steam",
    "battle.net",
    "galaxyclient",
    "skype",
    "uplay",
    "origin",
    "devenv",
    "taskmgr",
    "chrome",
    "discord",
    "firefox",
    "systemsettings",
    "applicationframehost",
    "cmd",
    "shellexperiencehost",
    "winstore.app",
    "searchui",
    "lockapp",
    "windowsinternal.composableshell.experiences.textinput.inputapp",
];

pub fn is_blacklisted_window(exe: &str) -> bool {
    let exe_lowercase = exe.to_lowercase();
    let exe_lowercase = exe_lowercase.trim_end_matches(".exe");

    BLACKLISTED_EXE
        .iter()
        .any(|&blacklisted_exe| blacklisted_exe == exe_lowercase)
}
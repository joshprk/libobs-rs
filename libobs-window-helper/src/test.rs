use crate::{get_all_windows, validators::WindowSearchMode, WindowInfo};

#[test]
pub fn test_iteration() {
    let res1 = get_all_windows(WindowSearchMode::ExcludeMinimized).unwrap();
    let res2 = get_all_windows(WindowSearchMode::IncludeMinimized).unwrap();

    log_res(res1);
    log_res(res2);
}

fn log_res(info: Vec<WindowInfo>) {
    #[cfg(feature = "serde")]
    {
        let json = serde_json::to_string_pretty(&info).unwrap();
        println!("{}", json)
    }
    #[cfg(not(feature = "serde"))]
    println!("{:?}", info);
}

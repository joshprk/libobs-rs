use crate::{get_all_windows, validators::WindowSearchMode, WindowInfo};


#[test]
pub fn test_iteration() {
    let res1 = get_all_windows(WindowSearchMode::ExcludeMinimized, false).unwrap();
    let res2 = get_all_windows(WindowSearchMode::ExcludeMinimized, true).unwrap();

    let res3 = get_all_windows(WindowSearchMode::IncludeMinimized, true).unwrap();
    let res4 = get_all_windows(WindowSearchMode::IncludeMinimized, false).unwrap();

    log_res(res1);
    log_res(res2);
    log_res(res3);
    log_res(res4);
}


fn log_res(info: Vec<WindowInfo>) {
    #[cfg(feature="serde")]
    {
        let json = serde_json::to_string_pretty(&info).unwrap();
        println!("{}", json)
    }
    #[cfg(not(feature="serde"))]
    println!("{:?}", info);
}
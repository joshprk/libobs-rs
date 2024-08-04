#[cfg(test)]
mod tests {
    use libobs_sources::windows::MonitorCaptureSourceBuilder;


    #[test]
    pub fn monitor_list() {
        MonitorCaptureSourceBuilder::get_monitors().unwrap();
    }
}

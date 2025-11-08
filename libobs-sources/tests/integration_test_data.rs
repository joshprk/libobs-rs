use libobs_wrapper::{context::ObsContext, utils::StartupInfo};

/// Integration test: Test encoder settings manipulation
#[test]
pub fn test_data_set() {
    let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
        .is_test(true)
        .try_init();

    let context = ObsContext::new(StartupInfo::default()).unwrap();
    let mut data = context.data().unwrap();

    // Test setting various data types
    data.set_int("bitrate", 5000).unwrap();
    data.set_bool("psycho_aq", true).unwrap();
    data.set_string("preset", "fast").unwrap();
    data.set_double("preset", 1.5).unwrap();

    // Verify we can retrieve them
    let bitrate = data.get_int("bitrate");
    assert_eq!(bitrate, Ok(5000));

    let psycho_aq = data.get_bool("psycho_aq");
    assert_eq!(psycho_aq, Ok(true));

    let preset = data.get_string("preset");
    assert_eq!(preset, Ok("fast".to_string()));

    let preset_double = data.get_double("preset");
    assert_eq!(preset_double, Ok(1.5));
}

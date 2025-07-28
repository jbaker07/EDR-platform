use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref UID_BASELINES: Mutex<HashMap<String, i32>> = {
        let mut m = HashMap::new();
        m.insert("/usr/bin/dbus-daemon".to_string(), 81); // example
        m.insert("/usr/lib/systemd/systemd".to_string(), 0);
        Mutex::new(m)
    };
}

pub fn get_baseline_uid(binary: &str) -> Option<i32> {
    UID_BASELINES.lock().unwrap().get(binary).cloned()
}

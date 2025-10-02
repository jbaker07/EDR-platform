use forensic_hooks::modules::telemetry_fingerprint;

fn main() {
    println!(
        "Running from: {}",
        std::env::current_dir().unwrap().display()
    );
    telemetry_fingerprint::crawl_all_advanced_and_write("src/modules/telemetry_fingerprint.json");
}

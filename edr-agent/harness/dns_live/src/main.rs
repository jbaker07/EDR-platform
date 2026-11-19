// harness/dns_live/src/main.rs
//
// Live kernel test for the DNS tunnel monitor.
//
//   dns_live <dns_tunnel_monitor.bpf.o> [--stop-on NAME] [--timeout SECS]
//     Loads the object and attaches the udp_sendmsg and udpv6_sendmsg kprobes
//     through the agent's own loader, then prints every captured datagram
//     with the parser result and the detector verdict. Stops after it sees a
//     query for NAME, or after SECS seconds (default 120).
//
//   dns_live --check-btf <btf file> <dns_tunnel_monitor.bpf.o>
//     Runs the CO-RE relocations against another kernel's BTF (for example
//     one built from the 5.15 vmlinux.h) and loads the result through this
//     kernel's verifier without attaching it. The relocation lines show which
//     offsets and enum values libbpf picked for that kernel.

// The agent's module, compiled as is. Not every item is used here.
#[allow(dead_code)]
#[path = "../../../src/modules/dns_tunnel.rs"]
mod dns_tunnel;

use std::process::ExitCode;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use dns_tunnel::{qtype_name, DnsSendEvent, DnsTunnelDetector};

const USAGE: &str = "usage: dns_live <obj> [--stop-on NAME] [--timeout SECS]\n       dns_live --check-btf <btf> <obj>";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 4 && args[1] == "--check-btf" {
        return match check_btf(&args[2], &args[3]) {
            Ok(()) => {
                println!("CHECK-BTF OK: relocated against {} and accepted by the verifier", args[2]);
                ExitCode::SUCCESS
            }
            Err(e) => {
                println!("CHECK-BTF FAILED: {e}");
                ExitCode::FAILURE
            }
        };
    }
    if args.len() < 2 || args[1].starts_with("--") {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    }

    let obj_path = &args[1];
    let mut stop_on: Option<String> = None;
    let mut timeout_secs: u64 = 120;
    let mut i = 2;
    while i < args.len() {
        let value = match args.get(i + 1) {
            Some(v) => v,
            None => {
                eprintln!("{USAGE}");
                return ExitCode::from(2);
            }
        };
        match args[i].as_str() {
            "--stop-on" => stop_on = Some(value.to_ascii_lowercase()),
            "--timeout" => timeout_secs = value.parse().unwrap_or(120),
            _ => {
                eprintln!("{USAGE}");
                return ExitCode::from(2);
            }
        }
        i += 2;
    }
    run_live(obj_path, stop_on, timeout_secs)
}

#[derive(Default)]
struct Counts {
    events: u64,
    ipv4: u64,
    ipv6: u64,
    parsed: u64,
    unparsed: u64,
    benign: u64,
    alerts: u64,
    repeats: u64,
}

fn run_live(obj_path: &str, stop_on: Option<String>, timeout_secs: u64) -> ExitCode {
    // The ring buffer callback runs on the loader's thread. It only forwards
    // events, and all printing and scoring happens here on the main thread.
    let (tx, rx) = mpsc::channel::<DnsSendEvent>();
    let started = dns_tunnel::start_dns_monitor(obj_path, move |event| {
        let _ = tx.send(*event);
    });
    if let Err(e) = started {
        println!("ATTACH FAILED: {e}");
        return ExitCode::FAILURE;
    }
    println!("READY: kprobes on udp_sendmsg and udpv6_sendmsg attached, reading ring buffer dns_events");

    let start = Instant::now();
    let deadline = start + Duration::from_secs(timeout_secs);
    let mut detector = DnsTunnelDetector::new();
    let mut counts = Counts::default();
    let mut stop_seen = false;

    while !stop_seen {
        let now = Instant::now();
        if now >= deadline {
            break;
        }
        let event = match rx.recv_timeout(deadline - now) {
            Ok(e) => e,
            Err(_) => break,
        };
        counts.events += 1;
        if event.family == 10 {
            counts.ipv6 += 1;
        } else {
            counts.ipv4 += 1;
        }
        println!(
            "[{}] pid={} tid={} uid={} comm={} family={} dport={} sent={} captured={}",
            counts.events,
            event.tgid,
            event.tid,
            event.uid,
            event.comm(),
            event.family_name(),
            event.dport,
            event.orig_len,
            event.cap_len
        );

        // Seconds since start only need to move forward for the volume window.
        match detector.process(&event, start.elapsed().as_secs()) {
            Err(e) => {
                counts.unparsed += 1;
                println!("    not parsed: {e}");
            }
            Ok(a) => {
                counts.parsed += 1;
                println!(
                    "    query {} type={} name_len={} longest_label={} labels={} subdomain_len={} entropy={:.2} unique_in_window={}",
                    a.name,
                    qtype_name(a.qtype),
                    a.features.name_len,
                    a.features.longest_label,
                    a.features.label_count,
                    a.features.subdomain_len,
                    a.features.subdomain_entropy,
                    a.unique_subdomains
                );
                if a.should_alert() {
                    counts.alerts += 1;
                    println!("    ALERT: {}", a.reason_text());
                } else if a.is_flagged() {
                    counts.repeats += 1;
                    println!("    flagged, already alerted for this process and base domain: {}", a.reason_text());
                } else {
                    counts.benign += 1;
                    println!("    benign");
                }
                if stop_on.as_deref() == Some(a.name.as_str()) {
                    stop_seen = true;
                }
            }
        }
    }

    println!(
        "SUMMARY events={} ipv4={} ipv6={} parsed={} unparsed={} benign={} alerts={} repeats={} stopped_by={}",
        counts.events,
        counts.ipv4,
        counts.ipv6,
        counts.parsed,
        counts.unparsed,
        counts.benign,
        counts.alerts,
        counts.repeats,
        if stop_seen { "stop marker" } else { "timeout" }
    );
    ExitCode::SUCCESS
}

fn check_btf(btf_path: &str, obj_path: &str) -> Result<(), String> {
    use libbpf_rs::libbpf_sys;
    use std::ffi::CString;

    libbpf_rs::set_print(Some((libbpf_rs::PrintLevel::Debug, print_core_lines)));
    let obj_c = CString::new(obj_path).map_err(|e| e.to_string())?;
    let btf_c = CString::new(btf_path).map_err(|e| e.to_string())?;
    // libbpf-rs 0.21 has no setter for btf_custom_path, so this calls libbpf directly.
    let opts = libbpf_sys::bpf_object_open_opts {
        sz: std::mem::size_of::<libbpf_sys::bpf_object_open_opts>() as libbpf_sys::size_t,
        btf_custom_path: btf_c.as_ptr(),
        ..Default::default()
    };
    // SAFETY: obj_c, btf_c and opts outlive both calls. The object pointer is
    // checked for null before use and closed exactly once.
    unsafe {
        let obj = libbpf_sys::bpf_object__open_file(obj_c.as_ptr(), &opts);
        if obj.is_null() {
            return Err(format!("open {obj_path} failed"));
        }
        let err = libbpf_sys::bpf_object__load(obj);
        libbpf_sys::bpf_object__close(obj);
        if err != 0 {
            return Err(format!("load returned {err}"));
        }
    }
    Ok(())
}

/// libbpf debug output cut down to the CO-RE relocation lines and any errors.
fn print_core_lines(_level: libbpf_rs::PrintLevel, msg: String) {
    if msg.contains("relo #") || msg.contains("CO-RE") || msg.contains("failed") || msg.contains("rror") {
        print!("{msg}");
    }
}

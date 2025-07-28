# trust_snapshot_scheduler.py

import time
import subprocess

INTERVAL_SECONDS = 3600  # 1 hour, adjust as needed

def run_census_collector():
    try:
        subprocess.run(["python3", "trust_census_collector.py"], check=True)
    except subprocess.CalledProcessError as e:
        print(f"[!] Trust census collection failed: {e}")

def main():
    while True:
        print("[⏱️] Running scheduled trust census collection...")
        run_census_collector()
        time.sleep(INTERVAL_SECONDS)

if __name__ == "__main__":
    main()

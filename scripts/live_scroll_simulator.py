#!/usr/bin/env python3
import time
import argparse
import webbrowser
import random
import pyautogui

def simulate_legit_behavior():
    print("[+] Simulating normal user behavior...")
    webbrowser.open("https://www.wikipedia.org", new=0)
    time.sleep(3)
    for _ in range(5):
        pyautogui.scroll(-300)
        time.sleep(random.uniform(0.5, 1.5))

def simulate_malicious_behavior():
    print("[!] Simulating malicious behavior...")
    webbrowser.open("https://example.com/malware-test", new=0)
    time.sleep(2)
    for _ in range(20):
        pyautogui.scroll(-500)
        time.sleep(0.2)

def interactive_mode():
    while True:
        choice = input("\nChoose behavior to simulate:\n[1] Legitimate\n[2] Malicious\n[Q] Quit\n> ").strip().lower()
        if choice == "1":
            simulate_legit_behavior()
        elif choice == "2":
            simulate_malicious_behavior()
        elif choice == "q":
            break
        else:
            print("Invalid input. Choose 1, 2, or Q.")

def auto_mode(cycle_seconds=30):
    while True:
        simulate_legit_behavior()
        print("[=] Sleeping before next cycle...\n")
        time.sleep(cycle_seconds)

        simulate_malicious_behavior()
        print("[=] Sleeping before next cycle...\n")
        time.sleep(cycle_seconds)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Simulate endpoint scroll behavior.")
    parser.add_argument("--mode", choices=["interactive", "auto"], default="interactive", help="Choose simulation mode.")
    parser.add_argument("--cycle", type=int, default=30, help="Cycle interval for auto mode in seconds.")
    args = parser.parse_args()

    if args.mode == "interactive":
        interactive_mode()
    else:
        auto_mode(args.cycle)

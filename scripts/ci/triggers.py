# Benign stand-ins for four playbooks and a TLS hello, for the live smoke test.
# Runs as root on a throwaway machine (a CI runner or a privileged container). Every
# file it touches is one it creates, and the only network traffic goes to 127.0.0.1.
import os, socket, ssl, subprocess, sys, threading, time

DEVNULL = subprocess.DEVNULL
TLS_NAME = "ci-smoke.example.test"


def step(name):
    print(f"{time.strftime('%H:%M:%S')} {name}", flush=True)


def run(*cmd):
    return subprocess.run(list(cmd), stdout=DEVNULL, stderr=DEVNULL).returncode


def reader_named(path):
    # A tiny C program that reads the file it is given, built under the name a playbook
    # watches for. A copy of /bin/cat does not work where cat is the Rust coreutils
    # multicall binary (Ubuntu 25.10 and later): it picks the tool from its own file
    # name, so a copy named mimipenguin prints an error and never opens the file.
    src = path + ".c"
    with open(src, "w") as f:
        f.write("#include <stdio.h>\n"
                "int main(int argc, char **argv) {\n"
                "    char line[256];\n"
                "    FILE *f = argc > 1 ? fopen(argv[1], \"r\") : NULL;\n"
                "    if (f == NULL) return 1;\n"
                "    while (fgets(line, sizeof line, f) != NULL) {}\n"
                "    fclose(f);\n"
                "    return 0;\n"
                "}\n")
    if run("gcc", "-O1", "-o", path, src) != 0:
        sys.exit("gcc could not build " + path)
    os.remove(src)


def renames(dirs, n, suffix):
    for d in dirs:
        os.makedirs(d, exist_ok=True)
    for i in range(n):
        d = dirs[i % len(dirs)]
        p = f"{d}/f{i}.txt"
        with open(p, "w") as f:
            f.write("data\n")
        os.rename(p, p + suffix)


def tls_hello():
    # A sink on 127.0.0.1:443 that never answers, and a client that sends its hello.
    srv = socket.socket()
    srv.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    srv.bind(("127.0.0.1", 443))
    srv.listen(4)
    threading.Thread(target=lambda: srv.accept(), daemon=True).start()
    ctx = ssl.create_default_context()
    ctx.check_hostname = False
    ctx.verify_mode = ssl.CERT_NONE
    raw = socket.create_connection(("127.0.0.1", 443))
    raw.settimeout(1.5)
    try:
        ctx.wrap_socket(raw, server_hostname=TLS_NAME)
    except (socket.timeout, ssl.SSLError, OSError):
        pass
    raw.close()
    srv.close()


step("lotl_cred: a file reader named mimipenguin reads /etc/shadow")
reader_named("/usr/local/bin/mimipenguin")
run("/usr/local/bin/mimipenguin", "/etc/shadow")

step("terminal: bash starts curl")
run("bash", "-c", "curl -s -m 1 http://127.0.0.1:9/; true")

step("tls: a ClientHello for " + TLS_NAME + " to 127.0.0.1:443")
tls_hello()

step("backup_wipe_first: rm -rf /backup/daily, then 400 renames to .locked")
os.makedirs("/backup/daily", exist_ok=True)
run("rm", "-rf", "/backup/daily")
time.sleep(1)
renames(["/srv/ci_data"], 400, ".locked")

step("fileburst_crypto: 510 renames to .enc in three top dirs, then openssl enc")
renames(["/home/ci_smoke/docs", "/srv/ci_share", "/opt/ci_app/data"], 510, ".enc")
run("openssl", "enc", "-aes-256-cbc", "-pbkdf2", "-pass", "pass:ci", "-in", "/etc/hostname", "-out", "/tmp/ci_smoke.enc")

step("done")
sys.exit(0)

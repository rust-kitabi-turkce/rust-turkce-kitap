import os
import subprocess
import sys
import threading
import webbrowser
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

ROOT = Path(__file__).resolve().parent
BOOK_DIR = ROOT / "book"
PORT = 8080


def build_book() -> None:
    subprocess.run(["mdbook", "build"], cwd=ROOT, check=True)


def serve_book() -> None:
    os.chdir(BOOK_DIR)
    handler = SimpleHTTPRequestHandler
    httpd = ThreadingHTTPServer(("", PORT), handler)
    httpd.daemon_threads = True
    url = f"http://localhost:{PORT}/"
    threading.Timer(0.5, lambda: webbrowser.open(url)).start()
    print(f"Serving book at {url} (Ctrl+C to stop)")
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        print("\nStopping server.")


def main() -> None:
    build_book()
    serve_book()


if __name__ == "__main__":
    try:
        main()
    except subprocess.CalledProcessError as exc:
        print(f"Build failed: {exc}", file=sys.stderr)
        sys.exit(1)
    except OSError as exc:
        print(f"Error: {exc}", file=sys.stderr)
        sys.exit(1)

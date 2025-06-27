# 🌀 purl — A Private, Lightweight `curl`-like HTTP Client

`purl` is a minimal, privacy-respecting HTTP client written in Rust. It works like `curl` — but faster, smaller, and with secure defaults. Ideal for scripting, CLI tools, and terminal-friendly services like `wttr.in`.

---

## 🚀 Features

- ✅ Type `purl wttr.in` — no need to type `https://`
- 🔒 HTTPS enforced (blocks `http://` unless `--unsecured`)
- 🧹 Strips tracking query params (like `utm_source`, `fbclid`)
- 🧭 SOCKS5 proxy support (e.g., for Tor)
- 📝 Show response headers with `-H`
- ⚡ Small, fast binary (~700 KB)

---

## 📦 Installation

### 🔧 Install with Cargo (recommended)

```bash
cargo install --git https://github.com/bearcry55/purl

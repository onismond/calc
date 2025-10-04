# Calc 🧮

A simple, fast, and lightweight CLI calculator built with Rust.

---

## 📦 Installation

### 🐧 Ubuntu / Debian
You can install the latest `.deb` package directly from Cloudsmith:

```bash
curl -1sLf 'https://dl.cloudsmith.io/public/onismond/calc/setup.deb.sh' | sudo -E bash
sudo apt install calc
```

To install a specific version (e.g., `0.1.0`):

```bash
curl -1sLf 'https://dl.cloudsmith.io/public/onismond/calc/setup.deb.sh' | sudo -E bash
sudo apt install calc=0.1.0
```

---

### 🏢 Enterprise Linux (RHEL / CentOS / AlmaLinux / Rocky)

For `.rpm`-based systems:

```bash
curl -1sLf 'https://dl.cloudsmith.io/public/onismond/calc/setup.rpm.sh' | sudo -E bash
sudo dnf install calc
```

To install a specific version (e.g., `0.1.0`):

```bash
curl -1sLf 'https://dl.cloudsmith.io/public/onismond/calc/setup.rpm.sh' | sudo -E bash
sudo dnf install calc-0.1.0
```

*(or use `yum install calc-0.1.0` on older systems)*

---

### 🪟 Windows

Download the latest prebuilt binary from Cloudsmith:

```
[https://dl.cloudsmith.io/public/onismond/calc/raw/versions/latest/calc.exe](https://dl.cloudsmith.io/public/onismond/calc/raw/versions/latest/calc.exe)
```

For a specific version (e.g., `0.1.0`):

```
[https://dl.cloudsmith.io/public/onismond/calc/raw/versions/0.1.0/calc.exe](https://dl.cloudsmith.io/public/onismond/calc/raw/versions/0.1.0/calc.exe)
```

Then, run it directly from the terminal:

```powershell
.\calc.exe
```

*(Optionally, add it to your PATH for easier use.)*

---

### 🍎 macOS

Download the latest macOS binary from Cloudsmith:

```
[https://dl.cloudsmith.io/public/onismond/calc/raw/versions/latest/calc](https://dl.cloudsmith.io/public/onismond/calc/raw/versions/latest/calc)
```

For a specific version (e.g., `0.1.0`):

```
[https://dl.cloudsmith.io/public/onismond/calc/raw/versions/0.1.0/calc](https://dl.cloudsmith.io/public/onismond/calc/raw/versions/0.1.0/calc)
```

Make it executable and move it to your PATH:

```bash
chmod +x calc
sudo mv calc /usr/local/bin/
```

---

## 🧰 Usage

Run the calculator from your terminal:

```bash
calc sum 2 5
```

Example output:

```
Result: 7
```

Show available commands:

```bash
calc --help
```

Example output:

```
A calculater with multiple operations

Usage: calc <COMMAND>

Commands:
  sum   Add two numbers
  sub   Substract two numbers
  mul   Multiply two numbers
  div   Divide two numbers
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

---

## 📝 License

This project is licensed under the [MIT License](LICENSE).

---

## 💡 About

Created by **Onismond Yao Duame** 
Source code: [https://github.com/onismond/calc](https://github.com/onismond/calc)

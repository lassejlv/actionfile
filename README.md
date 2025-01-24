# Actionfile

A very fast command runner, wich looks a lot like npm scripts. But this has it own config-scripts syntax. Written in rust as a learning project.

# Features

- Understands npm scripts & deno tasks.
- Easy config language
- Package-manager detector. Will automatically detect if you are using npm, pnpm, deno or bun. Even works with python, go, cargo and more. **SOON**
- Extremely fast. No more headaches waiting for your scripts to run.

## Quick Install

> **Note**: Currently only supports macOS. Linux support coming soon!

Install actionfile with this one-liner:

```bash
curl -fsSL https://raw.githubusercontent.com/lassejlv/actionfile/main/scripts/install.sh | bash
```

## Uninstall

To uninstall actionfile, run:

```bash
curl -fsSL https://raw.githubusercontent.com/lassejlv/actionfile/main/scripts/uninstall.sh | bash
```

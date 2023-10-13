# URL Defanger
Prevent unintentional clicks to malicious websites by defanging URLs when analyzing malicious documents

### CI Pipeline Status
[![CircleCI](https://dl.circleci.com/status-badge/img/gh/fdvmoreira/url-defanger/tree/main.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/gh/fdvmoreira/url-defanger/tree/main)


## How to run this project locally?

## Dependencies
```yaml
rust = 1.60  
yew = 0.20
```


1. Download or clone the project. Unpack and *cd* into the directory
```bash
git clone https://github.com/fdvmoreira/url-defanger.git
```

2. Install [rustup](https://rustup.rs)
Download:
- Linux:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- Windows (installer): https://win.rustup.rs/x86_64

3. Install [trunk](https://trunkrs.dev) with Cargo
```bash
cargo install trunk
```

4. Run trunk to serve the application
```bash
trunk serve
```

## LICENSE
Licensed under MIT [LICENSE](./LICENSE.md).

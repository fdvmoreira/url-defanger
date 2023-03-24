# url-defanger
Defange URL to remove armfull links from your documents when dealing analysing malicious documents

## prerequisites
```
rust = 1.60
yew = 0.20

```
## How to run this project locally ?

1. Download or clone the project. Unpack and *cd* into the directory
```
git clone https://github.com/fdvmoreira/url-defanger.git
```

2. Install [rustup](https://rustup.rs)
Download:
	- Linux: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  - Windows (installer): https://win.rustup.rs/x86_64

3. Install [trunk](https://trunkrs.dev) with Cargo
```
cargo install trunk
```

4. Run trunk to serve the application
```
trunk serve
```

### How it works?
1. Type in the URL you want to defange click the defange button or press ENTER
2. Copy the output by click button or select the text
---

## LICENSE
MIT
Read the [LICENSE](./LICENSE.md)

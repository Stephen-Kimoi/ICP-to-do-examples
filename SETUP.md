# **Setup**

## **Prerequisites**  
To follow these tutorials, you will need the following:  

- A connection to the internet.  
- A command line interface.  
- [Visual Studio (VS) Code](https://code.visualstudio.com/).  

Ensure you have these installed on your machine:  
- [Node.js 18](https://nodejs.org/)  
- [dfx](https://internetcomputer.org/docs/current/developer-docs/setup/install/)  
- [Rust](https://www.rust-lang.org/)  

Below are the links and commands you can run in your terminal depending on the type of OS you are using (Linux, Windows, or macOS).

---

## **Linux**

### **Node.js Installation**  
To install Node.js v18, run these commands in your terminal:

```bash
# Update your package list
sudo apt update

# Install required dependencies
sudo apt install -y curl

# Add Node.js 18 PPA
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -

# Install Node.js
sudo apt install -y nodejs

# Verify the installation
node -v
npm -v
```

Or refer to the [Node.js package manager guide](https://nodejs.org/en/download/package-manager).

---

### **DFX Installation**  
To install `dfx`, run these commands in your terminal:

```bash
# Download and install dfxvm
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"

# Reload your shell configuration
source ~/.bashrc

# Install the latest dfx version
dfxvm install 0.18.0

# Set the default dfx version
dfxvm default 0.18.0

# Verify the installation
dfx --version
```

---

### **Rust Installation**  
To install Rust, run the following commands:

```bash
# Download and run the Rust installation script
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Ensure Rust is in your system path
source $HOME/.cargo/env

# Verify the installation
rustc --version
cargo --version

# Add support for compiling Rust code to WebAssembly (Wasm)
rustup target add wasm32-unknown-unknown
```

---

## **Windows**  
We recommend using **WSL 2** (Windows Subsystem for Linux).  

### **Installing WSL**  
Run the following command in PowerShell or Command Prompt:  
```bash
wsl --install
```

Once WSL has been installed, open the Ubuntu terminal and follow the Linux installation instructions above.

---

## **macOS**

### **General Requirements**  
Ensure you are running macOS 12.* Monterey or later.  

If you are using a machine with Apple Silicon, install Rosetta by running:  
```bash
softwareupdate --install-rosetta
```

### **Install Xcode Command Line Tools**  
Run the following command:  
```bash
xcode-select --install
```

---

### **Node.js Installation**  
Use Homebrew to install Node.js v18:  
[Node.js via Homebrew](https://formulae.brew.sh/formula/node@18)  

---

### **DFX Installation**  
Run this command in your terminal:  
```bash
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
```

---

### **Rust Installation**  
Follow this [installation tutorial](https://medium.com/@Brahmbhatnilay/rust-programming-language-installation-for-beginners-on-windows-and-mac-47bf7fdcbf80).  

---

## **Uninstalling DFX and DFXVM**

To uninstall `dfx` and `dfxvm`, run the following commands:

```bash
dfxvm self uninstall
```

If prompted for confirmation, use:  
```bash
dfxvm self uninstall --yes
```
```
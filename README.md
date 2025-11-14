It is incredibly annoying to constantly google .gitignore templates for langauges!

So this CLI is meant to very quickly create .gitignore file in the current directory. 

# Usage

Very simply:
```
ignore python
```
will create a .gitignore file for python.

```
ignore rust
```
will create a .gitignore for rust. 

```
ignore r
```

```
```
will create a .gitignore for R and so on and so forth.

If you're working on a project that handles multiple languages then you can just call each one and it'll append to the .gitignore.



# Installation	

## Downloading the binary
The binary for `ignore` is only available on mac-os but will be made available on Linux soon. 

**Installing ignore is very easy**
```
curl -L -o ignore https://github.com/trystanscottlambert/ignore/releases/download/v0.1.0/ignore
chmod +x ignore
sudo mv ignore /usr/local/bin/
```

You may need to start a new terminal to get it working.

If you don't want to install the binary then you can compile the program from source using 'cargo'. 

## Compile from source

If you are using Linux or don't want to download a binary file then `ignore` can be built from source using cargo. 

First make sure you have rust installed:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Download the ignore repo with git
```
git clone git@github.com:TrystanScottLambert/ignore.git
```

cd into the ignore folder
`cd ignore/`
You should be able to see the `Cargo.toml` file. From here compile using cargo (which would already be installed with rust.)
```
cargo build --release
```

Then simply move the binary file into your /bin directory

```
sudo mv target/release/ignore /usr/local/bin
```

You may need to restart the terminal. 

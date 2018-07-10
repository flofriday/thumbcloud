# Thumbcloud
Thumbcloud is a drop-in software replacement for your USB thumbdrive.
![Screenshot](./screenshot.png)

## Install
Just download the [latest release](https://github.com/flofriday/thumbcloud/releases) and unpack it.

Warning: Windows and macOS might warn you because the binaries are not singed.
Feel free ignore those warnings.

### Build from source
1. [Install rust](https://doc.rust-lang.org/book/second-edition/ch01-01-installation.html)
2. [Download the repository](https://github.com/flofriday/thumbcloud/archive/master.zip)
3. Unzip the compressed file and run `cargo build --release`
4. Run the executable with `./target/release/thumbcloud $HOME` (on Windows: `.\target\release\thumbcloud.exe %HOMEPATH%` )

## Usage
1. Open a terminal in the directory with the executable
2. Windows: `.\thumbcloud.exe %HOMEPATH%`
   Unix: `./thumbcloud $HOME` (if it fails because of a permission `sudo ./thumbcoud $HOME`)
3. Open a Webbrowser and type `localhost:8080` in the addressbar

## Todo List 
### v0.0.1
- [X] Viewing files in the webbrowser

### v0.0.2
- [X] Download files from the webbrowser
- [X] "fancy" aka useable Web UI
- [X] Show filesize
- [X] About page
- [ ] Let the user selecte the shared folder
- [X] Parse commandline arguments

### v0.0.3
- [ ] Upload files
- [ ] Download folders as .zip
- [ ] View file content without downloading
- [ ] System page
- [ ] 404 page

### after v0.0.3
- [ ] GUI for server
- [ ] Add Logo
- [ ] Product Website
- [ ] Server Settings
    - [ ] Set max connections
    - [ ] Restrict to only download, only view files
    - [ ] Only access with password

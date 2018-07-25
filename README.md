# Thumbcloud
Thumbcloud is a file-sharing server to replace your USB thumbdrive
![Screenshot](./screenshot.png)

## Idea and State
Sharing files via a USB thumbdrive is not optimal:
* only one user can access the content at a time
* you have to carry a physical object (which you will for sure forget when you need it the most, thanks to [Murphy's law](https://en.wikipedia.org/wiki/Murphy%27s_law))
* USB connections are "slow"
* not all modern computers have USB type A ports
* USB drives are limited in size (often just a few GB)
Thumbcloud is my attempt to create an application to share files with friends and collegues, on the 
same network. Moreover, they don't need to install any additional software (all they need is a 
webbrowser).<br>
<br>
**Disclaimer** : The software still is in early development and misses some important features.
Furthermore, there are some security flaws (like path traversal). 
In a nutshell, you shouldn't use the software yet in "production", but feel free to try it out.

## Install
Just download the [latest release](https://github.com/flofriday/thumbcloud/releases) and unpack it.

Warning: Windows and macOS might warn you because the binaries are not singed.
Feel free ignore those warnings.

### Build from source
1. [Install rust](https://doc.rust-lang.org/book/second-edition/ch01-01-installation.html)
2. [Download the repository](https://github.com/flofriday/thumbcloud/archive/master.zip)
3. Unzip the compressed file and run `cargo build --release`
4. Run the executable<br> 
   Windows PowerShell: `.\target\release\thumbcloud.exe $HOME`<br>
   Windows CMD: `.\target\release\thumbcloud.exe %HOMEPATH%`<br>
   Unix (macOS, Linux, FreeBSD): `./target/release/thumbcloud $HOME`
5. Open a Webbrowser and type `localhost:8080` in the addressbar<br>
<br>
Tipp: Run `./thumbcloud --help` for more information.

## Usage
1. Open a terminal in the directory with the executable
2. Windows CMD: `.\thumbcloud.exe %HOMEPATH%`<br>
   Windows PowerShell: `.\thumbcloud.exe $HOME`<br>
   Unix (macOS, Linux FreeBSD): `./thumbcloud $HOME`
3. Open a Webbrowser and type `localhost:8080` in the addressbar<br>
<br>
Tipp: Run `./thumbcloud --help` for more information.

## Todo List 
### v0.0.1
- [X] Viewing files in the webbrowser

### v0.0.2
- [X] Download files from the webbrowser
- [X] "fancy" aka useable Web UI
- [X] Show filesize
- [X] About page
- [X] Let the user selecte the shared folder
- [X] Parse commandline arguments

### v0.0.3
- [ ] Upload files
- [ ] File icons dependent on filetype
- [X] Template engine for HTML files
- [ ] System page
- [ ] 404 page

### v0.1.0
- [ ] Optimize for mobile devices
- [ ] GUI for server
- [ ] Drag & Drop support
- [ ] Add Logo
- [ ] Product Website

### after v0.1.0
- [ ] Download folders as .zip
- [ ] View file content without downloading
- [ ] Server Settings
    - [ ] Set max connections
    - [ ] Restrict to only download, only view files
    - [ ] Only access with password

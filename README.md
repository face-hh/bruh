# BRUHIFF
**B**lazingly **r**apid **u**ncompressed **h**arebrained Image File Format.  

[![](https://img.shields.io/badge/status-stupidly_stupid-orange?style=plastic)](#)&nbsp;&nbsp;[![](https://img.shields.io/badge/python-is_cool-black?style=plastic&logo=python&logoColor=white)](#)&nbsp;&nbsp;[![](https://img.shields.io/badge/rust-is_fine_too-darkred?style=plastic&logo=rust&logoColor=white)](#)

Also known as BRUHIFF or BRUH.

<img width="659" height="709" alt="image" src="https://github.com/user-attachments/assets/ba39bd4f-61cd-4ca8-9559-e156f8b00fb4" />

# How to | Rust
1. Download the repo / 
2. Open a command prompt in the directory / `cd bruh`
3. Run `cargo run compile` followed by a `path/to/image.png` to compile PNG to BRUH.
DISPLAY BRUH IMAGE: Run `cargo run` followed by a `path/to/image.bruh` to show the image.
Examples:
- `cargo run compile C:\Users\User\Downloads\image.png` to convert a PNG to BRUH.
- `cargo run C:\Users\User\Downloads\image.bruh` to display the BRUH image.

# How to | Python
1. Download the repo / `git clone` it.
2. Open a command prompt in the directory / `cd bruh`
3. Run `python3 -m pip install -r requirements.txt` to install the requirements if you haven't already (NumPy, Pillow)
4. Run `python3 bruhiff.py` followed by a `path/to/image`, which can be either .bruh or .png, and it'll convert from one to the other.
Examples:
- `python3 bruhiff.py C:\Uses\User\Downloads\image.png` to convert a PNG to BRUH.
- `python3 bruhiff.py C:\Uses\User\Downloads\image.bruh` to convert a BRUH to PNG. (Although You Probably Won't need that 😉)

# How to | JavaScript
You can render .bruh images in websites. for some reason.
1. Download `bruh.min.js` / `bruh.js`.
2. Import the Script to your website using `<script src="path/to/bruh.min.js"></script>`.
3. Use `<img>` tag to display .bruh images as if you were to display a normal image.

## Add Default Image Viewer to .bruh Images
1. Double-click on `image.bruh` using your File Explorer.
2. Click on `More Apps`

<img width="392" height="497" alt="image" src="https://github.com/user-attachments/assets/c58b5e41-a939-45d5-9b03-b4899b506e98" />

3. Click on `Choose app from this PC`

<img width="391" height="518" alt="image" src="https://github.com/user-attachments/assets/c9990ea9-4d86-4c59-8d1f-1fa41d928fa8" />

Tip: tick "Always use this app to open .bruh files"

4. Type the `path/to/this/project`.
5. Select `bruh.exe` inside this folder.

That's it! You can now open `.bruh` files!

# Known issues | Rust
The PNG > BRUH won't work unless you have the same file (i.e. image.png) but with the .bruh extension (i.e. image.bruh). What do you have to do? Create an empty file called `image.bruh`.

1. Preview window width & height are not exact.
2. Huge file size on large images.
3. Slow preview window.
4. Some large images might include `#0` hex which will crash the program.
5. No transparency.
6. Only works on Windows

# Known issues | Python
Install the goddamn requirements and stop yapping

# Known issues | JavaScript
It does not work when you load using the file:/// protocol,  
JavaScript rendering requires a local server because browsers block fetch on the file:/// protocol.

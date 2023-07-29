# BRUHIFF
**B**lazingly **r**apid **u**ncompressed **h**arebrained Image File Format.

Also known as BRUHIFF or BRUH.

![Example](https://cdn.discordapp.com/attachments/1074408238939906220/1130764354661384192/image.png)

# How to
1. Download the repo / `git clone` it.
2. Open a command prompt in the directory / `cd bruh`
3. Run `cargo run compile` followed by a `path/to/image.png` to compile PNG to BRUH. Example: `cargo run compile C:\Uses\User\Downloads\image.png`

4. Run `cargo run` followed by a `path/to/image.bruh` to show the image

## OR
1. Double-click on `image.bruh` using your File Explorer.
2. Click on `More Apps`

![More Apps](https://cdn.discordapp.com/attachments/1074408238939906220/1130765375693406258/image.png)

3. Click on `Choose app from this PC`

![Choose app](https://cdn.discordapp.com/attachments/1074408238939906220/1130765548813308034/image.png)

Tip: tick "Always use this app to open .bruh files"

4. Type the `path/to/this/project`.
5. Select `bruh.exe` inside this folder.

That's it! You can now open `.bruh` files!

# Known issues
⚠ The PNG > BRUH won't work unless you have the same file (i.e. image.png) but with the .bruh extension (i.e. image.bruh). What do you have to do? Create an empty file called `image.png`.

1. Preview window width & height are not exact.
2. Huge file size on large images.
3. Slow preview window.
4. Some large images might include `#0` hex which will crash the program.
5. No transparency.
6. Only works on Windows

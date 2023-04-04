# Image to Clipboard - MacOS only


This app allows you to copy an image file to the clipboard on a Mac.
It uses the `cocoa` library to interact with the macOS Cocoa API.

## How to use

The program takes a single argument, which is the path to the image file you want to copy. For example:

```bash
image-to-clipboard /path/to/image.jpg
```


If the image file exists, it will be copied to the clipboard. If the file does not exist, an error message will be displayed. Exit code: `2`

You can also use the -h option to display the help message:

```
image-to-clipboard <path>
image-to-clipboard -h - show help
```

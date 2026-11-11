# imgresizer — Lightweight Batch Image Resizer

A small, dependency-free Windows utility for batch-resizing images from the command line.
Built for people who just want `input folder → output folder` without installing a 300 MB suite.

## Features

- Batch resize JPEG / PNG / BMP to a target width (aspect ratio preserved)
- Optional quality setting for JPEG output (`-q 1-100`, default 90)
- Recursive folder scanning (`-r`)
- Dry-run mode to preview what would be written (`--dry-run`)
- Single static binary, no runtime installer, no .NET requirement

## Usage

```text
imgresizer.exe <input> <output> [-w WIDTH] [-q QUALITY] [-r] [--dry-run]

  input          File or folder containing images
  output         Destination file or folder
  -w WIDTH       Target width in pixels (default 1920)
  -q QUALITY     JPEG quality 1-100 (default 90)
  -r             Scan subfolders recursively
  --dry-run      List planned conversions without writing
```

### Examples

```text
imgresizer.exe C:\Photos D:\Out -w 1600
imgresizer.exe .\screenshots .\web -w 800 -q 75 -r
imgresizer.exe wedding.jpg wedding_small.jpg -w 1200
```

## Building

Requires Rust (stable) with the MSVC toolchain:

```text
cargo build --release
```

## How it works

The resize core uses a bilinear filter over a raw RGB buffer. WIC (Windows Imaging
Component) is used only for decode/encode so the binary stays small and fast. No
external image libraries, no GPU dependency.

## License

MIT

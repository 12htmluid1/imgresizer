# imgresizer release notes

## 0.3.1
- Fix: recursive scan now skips symlinks to avoid loops
- Fix: quality flag clamped to 1-100 range

## 0.3.0
- Added `--dry-run` preview mode
- Added recursive folder scanning (`-r`)
- Switched release profile to `opt-level = "z"` (binary ~180 KB smaller)

## 0.2.0
- PNG output support
- Aspect-ratio-preserving resize is now the default behavior

## 0.1.0
- Initial release: JPEG batch resize, single static binary

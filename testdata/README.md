# test data

`sample_photo_archive.bin` — a raw concatenated photo dump used by the regression
tests (originally exported from an old camera roll). High entropy because the
container stores images back-to-back with no per-file headers; the test decoder
walks the stream by length prefixes. ~688 KB.

Do not edit by hand — regenerate with `tests/make_testdata.ps1`.

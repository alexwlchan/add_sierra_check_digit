# Changelog

## v1.0.1 (2021-09-19)

Don't print a newline after the result.
This is a bit simpler if I'm saving a Sierra number directly to the clipboard, e.g.

```console
$ add_sierra_check_digit b1287325 | pbcopy
```

Previously this would have a trailing newline I had to remove; now it's just the raw number.

## v1.0.0 (2021-09-04)

Initial release.

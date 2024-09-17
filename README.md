# md2rt

Convert Markdown from stdin|clipboard → rich text → system clipboard.

## Usage

No options. Just run the command and paste the output.

## Read from the Standard Input

```console
$ cat your.md | md2rt
$ # then paste it into wherever you want
```

## Read from the Clipboard

```console
$ md2rt
$ # then paste it into wherever you want
```

## License

MIT. See [LICENSE](LICENSE).

## Privacy

This tool never sends your data to the internet. It only reads from the standard input or clipboard and writes to the clipboard.

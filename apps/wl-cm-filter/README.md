# wl-cm-filter

This application can be used to limit capabilities reported by wp-color-management-v1

```
Usage: wl-cm-filter [OPTIONS] [PROGRAM]...

Arguments:
  [PROGRAM]...
          The program to run

Options:
      --generate-completion <SHELL>
          Generate shell completions instead of running the program
          
          [possible values: bash, elvish, fish, powershell, zsh]

  -i, --invert
          Invert the selection.
          
          If this flag is used, only those values that are listed in a filter are passed through.

      --render-intents <RENDER_INTENTS>
          The intents to filter

      --features <FEATURES>
          The features to filter

      --primaries <PRIMARIES>
          The primaries to filter

      --transfer-functions <TRANSFER_FUNCTIONS>
          The transfer functions to filter

  -h, --help
          Print help (see a summary with '-h')
```

## Example

```shell
$ wl-cm-filter --transfer-functions bt1886,srgb,gamma22 wayland-info
```

## License

This application is free software licensed under the GNU General Public License
v3.0.

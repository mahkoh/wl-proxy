# wl-paper

This application can be used to run any regular wayland application as a
layer-shell application.

```
Usage: wl-paper [OPTIONS] [PROGRAM]...

Arguments:
  [PROGRAM]...  The program to run

Options:
      --generate-completion <SHELL>
          Generate shell completions instead of running the program [possible values: bash, elvish, fish, powershell, zsh]
      --margin-top <MARGIN_TOP>
          The top margin [default: 0]
      --margin-right <MARGIN_RIGHT>
          The right margin [default: 0]
      --margin-bottom <MARGIN_BOTTOM>
          The bottom margin [default: 0]
      --margin-left <MARGIN_LEFT>
          The left margin [default: 0]
      --keyboard-interactivity <KEYBOARD_INTERACTIVITY>
          The keyboard interactivity [default: on-demand] [possible values: none, exclusive, on-demand]
      --layer <LAYER>
          The layer [default: background] [possible values: background, bottom, top, overlay]
  -h, --help
          Print help
```

## License

This application is free software licensed under the GNU General Public License
v3.0.

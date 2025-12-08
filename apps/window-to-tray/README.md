# window-to-tray

This application can be used to turn any regular wayland application into a tray
application. The following protocols must be supported by the compositor:

- jay-tray-v1
- jay-popup-ext-v1

```
Usage: window-to-tray [OPTIONS] [PROGRAM]...

Arguments:
  [PROGRAM]...  The program to execute

Options:
      --generate-completion <SHELL>  Generate shell completions instead of running the program [possible values: bash, elvish, fish, powershell, zsh]
      --border-color <BORDER_COLOR>  Set the color of the popup border [default: 000000]
      --border-width <BORDER_WIDTH>  Set the width of the popup border [default: 20]
      --icon-theme <ICON_THEME>      Set the icon theme used for icons using a symbolic name [default: hicolor]
      --icon-color <ICON_COLOR>      Set the color used for recolorable SVG icons [default: c8c8c8]
  -h, --help                         Print help
```

## License

This application is free software licensed under the GNU General Public License
v3.0.

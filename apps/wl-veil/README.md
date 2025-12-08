# wl-veil

This application can be used to hide wayland globals from applications or to
downgrade their versions.

```
Usage: wl-veil [OPTIONS] [PROGRAM]...

Arguments:
  [PROGRAM]...
          The program to run

Options:
      --generate-completion <SHELL>
          Generate shell completions instead of running the program
          
          [possible values: bash, elvish, fish, powershell, zsh]
          
    -i, --invert
          Invert the selection.
          
          If this flag is used, only those globals that are listed in a filter are passed through.

  -f <FILTER>
          The filters to apply.
          
          Each filter should either be `<global_name>` to filter the global
          outright or `<global_name>=<version>` to downgrade the global to that
          version.

  -h, --help
          Print help (see a summary with '-h')
```

## Example

```shell
$ wl-veil -f wp_linux_drm_syncobj_manager_v1 obs
```

## License

This application is free software licensed under the GNU General Public License
v3.0.

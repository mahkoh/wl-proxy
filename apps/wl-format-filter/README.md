# wl-format-filter

This application can be used to hide buffer formats from applications.

```
Filters can be specified with the --allow and --deny options.

For each format/modifier pair, the behavior is as follows:

- If it matches an allow rule, it is passed through.
- Otherwise, if it matches a deny rule, it is filtered out.
- Otherwise, it is passed through.

wl_shm formats implicitly use the linear modifier.
wl_drm and older zwp_linux_dmabuf_v1 versions implicitly use the invalid modifier.

Each filter should have one of the following formats:

- <format>
- <format>:<modifier>

<format> should have one of the following formats:

- `all` - to match all formats
- one of the format names from wayland.xml
- a four-character fourcc name
- a hexadecimal number prefixed with 0x
- a decimal number

If <modifier> is not specified, the filter applies to all modifiers.
Otherwise, <modifier> should have one of the following formats.

- `linear` - to match the linear modifier
- `invalid` - to match the invalid modifier
- a hexadecimal number prefixed with 0x
- a decimal number

Usage: wl-format-filter [OPTIONS] [PROGRAM]...

Arguments:
  [PROGRAM]...
          The program to run

Options:
      --generate-completion <SHELL>
          Generate shell completions instead of running the program
          
          [possible values: bash, elvish, fish, powershell, zsh]

  -a, --allow <FILTER>
          The formats to allow unconditionally

  -d, --deny <FILTER>
          The formats to deny unless they are explicitly allowed

  -h, --help
          Print help (see a summary with '-h')
```

## Example

```shell
$ wl-format-filter -a all:linear -d all wayland-info
```

## License

This application is free software licensed under the GNU General Public License
v3.0.

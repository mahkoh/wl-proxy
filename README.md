# wl-proxy

This repository contains the [wl-proxy](wl-proxy) rust crate that can be used to
proxy wayland connections and intercept and manipulate wayland messages.

See the documentation linked in the wl-proxy readme for more information about
the crate.

This repository furthermore contains a number of applications written with
wl-proxy:

- [wl-veil](apps/wl-veil): Can be used to hide globals or downgrade global
  versions.
- [wl-format-filter](apps/wl-format-filter): Can be used to hide buffer formats
  or modifiers.
- [wl-paper](apps/wl-paper): Can be used to run arbitrary applications as
  layer-shell applications.
- [window-to-tray](apps/window-to-tray): Can be used to run arbitrary
  applications as tray applications.

## License

Everything in this repository is licensed under the GNU General Public License
v3.0 except the wl-proxy crate itself which is dual licensed under the MIT and
Apache 2 licenses.

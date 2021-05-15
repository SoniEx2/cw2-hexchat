CW2 - Content Warnings for Hexchat
==================================

This is the best CW plugin for Hexchat. It turns messages like:

```text
<Soni> [CW hello] world
```

into:

```text
* [Content Hidden: hello] (Copy and paste this line to expand)
<Soni> [Content Hidden: hello]
```

and upon copying and pasting:

```text
* [Content Hidden: hello] world (Copy and paste this line to expand)
```

thus providing full CW functionality in a backwards-compatible, non-annoying
way, without any of the troubles brought by color code- and CTCP-based
CW solutions.

You can also write CWs using the `/cw` command, like so:

```text
/cw [This is the CW reason] this is the CW text
```

which becomes:

```text
<Soni> [CW This is the CW reason] this is the CW text
```

(so, really, you might be better off *not* using `/cw` and just typing the
message directly. :p Just don't add a `:` after `[CW`, it needs to be a
space...)

Building and Platform Support
-----------------------------

Make sure you have Rust installed. Just do `cargo build` (or optionally
`cargo build --release`). This should produce a libcw2.so in `target/debug`
(or `target/release`), which can be copied over to `hexchat/addons`.

The plugin has been tested on Linux, using Rust 1.52.1. It *should* work on
Windows, but that hasn't been tested.

License
-------

```text
CW2 - Hexchat Plugin for Content Warnings
Copyright (C) 2021  Soni L.

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```

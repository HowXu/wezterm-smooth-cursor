# Wez's Terminal

<img height="128" alt="WezTerm Icon" src="https://raw.githubusercontent.com/wezterm/wezterm/main/assets/icon/wezterm-icon.svg" align="left"> *A GPU-accelerated cross-platform terminal emulator and multiplexer written by <a href="https://github.com/wez">@wez</a> and implemented in <a href="https://www.rust-lang.org/">Rust</a>*

## Fork: smooth cursor trail

This repository is a fork of [WezTerm](https://github.com/wez/wezterm) that adds
a smooth cursor trail animation. The cursor trail follows cursor movement in the
active pane, keeps the trail behind the current movement direction, and is tuned
to make one-cell cursor moves visible while typing or moving around editors such
as Vim/Neovim.

The upstream WezTerm documentation still applies unless noted otherwise.

### Configuration

The feature is configured through `cursor_trail` in `wezterm.lua`:

```lua
local wezterm = require 'wezterm'
local config = wezterm.config_builder()

config.cursor_trail = {
  enabled = true,
  dwell_threshold = 0,
  duration = 80,
  spread = 2.6,
  distance_threshold = 1,
  opacity = 0.55,
}

return config
```

Available options:

| Option | Type | Default | Description |
| --- | --- | --- | --- |
| `enabled` | boolean | `true` | Enables or disables the cursor trail effect. |
| `dwell_threshold` | integer milliseconds | `0` | Delays trail target updates until the cursor has been stationary for this long. `0` updates the target immediately when the cursor moves. |
| `duration` | integer milliseconds | `80` | Animation duration for the leading edge to reach the cursor. Must be greater than `0`. |
| `spread` | number | `2.6` | Duration multiplier for the trailing edge. Higher values create a longer smear. Must be at least `1.0`. |
| `distance_threshold` | integer cells | `1` | Minimum cursor movement needed to draw the trail. Smaller moves snap directly to the new cursor position. |
| `opacity` | number | `0.55` | Cursor trail opacity. Must be between `0.0` and `1.0`. |

The trail frame cadence follows WezTerm's existing `animation_fps` setting.

User facing docs and guide at: https://wezterm.org/

![Screenshot](docs/screenshots/two.png)

*Screenshot of wezterm on macOS, running vim*

## Installation

https://wezterm.org/installation

## Getting help

This is a spare time project, so please bear with me.  There are a couple of channels for support:

* You can use the [GitHub issue tracker](https://github.com/wezterm/wezterm/issues) to see if someone else has a similar issue, or to file a new one.
* Start or join a thread in our [GitHub Discussions](https://github.com/wezterm/wezterm/discussions); if you have general
  questions or want to chat with other wezterm users, you're welcome here!
* There is a [Matrix room via Element.io](https://matrix.to/#/#wezterm:matrix.org)
  for (potentially!) real time discussions.

The GitHub Discussions and Element/Gitter rooms are better suited for questions
than bug reports, but don't be afraid to use whichever you are most comfortable
using and we'll work it out.

## Supporting the Project

If you use and like WezTerm, please consider sponsoring it: your support helps
to cover the fees required to maintain the project and to validate the time
spent working on it!

[Read more about sponsoring](https://wezterm.org/sponsor.html).

* [![Sponsor WezTerm](https://img.shields.io/github/sponsors/wez?label=Sponsor%20WezTerm&logo=github&style=for-the-badge)](https://github.com/sponsors/wez)
* [Patreon](https://patreon.com/WezFurlong)
* [Ko-Fi](https://ko-fi.com/wezfurlong)
* [Liberapay](https://liberapay.com/wez)

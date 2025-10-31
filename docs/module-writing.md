# Module Writing Guide

1. Copy `modules/example-app` or `modules/example-system-tweak` to start a new module.
2. Edit `module.toml` with a unique `id`, human-friendly `name`, and correct `script_kind`.
3. Update the `[compat]` section to list supported distros and desktops for your script.
4. Replace each script in `scripts/` with real automation, keeping the same file names.
5. Toggle `enabled = true` once you are confident the module should load by default.

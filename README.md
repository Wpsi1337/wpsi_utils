# wpsi_utils

Prototype utility collection inspired by `linutil`. This repository ships with a bootstrap script so the project can be launched via `curl -fsSL <url> | sh` once it is hosted (for example on GitHub or a personal server). Add your own scripts and configuration under `scripts/` and `config/`.

## Layout
- `bootstrap.sh` — lightweight installer invoked by the `curl | sh` entrypoint.
- `bin/wpsi_utils.sh` — main runner; load or invoke your utilities here.
- `scripts/` — place standalone helper scripts. An example is provided.
- `config/` — store optional configuration files consumed by your scripts.

## Usage
1. Host `bootstrap.sh` somewhere accessible (e.g. GitHub `raw` URL or a web server).
2. On the target system run:
   ```sh
   curl -fsSL https://example.com/wpsi_utils/bootstrap.sh | sh
   ```
3. The bootstrapper clones the git repository (defaults to the `main` branch), executes `bin/wpsi_utils.sh`, and passes along any extra arguments.

Override the repository URL or branch without editing the script:
```sh
curl -fsSL https://example.com/wpsi_utils/bootstrap.sh | \
  WPSI_UTILS_REPO_URL=https://github.com/YOURNAME/wpsi_utils.git \
  WPSI_UTILS_BRANCH=main \
  sh -s -- --flag
```

## Next Steps
- Build your actual tooling inside `bin/wpsi_utils.sh` or split it into modules under `scripts/`.
- Replace the placeholder repository URL inside `bootstrap.sh` once you publish the project.
- Optionally cut releases and swap the bootstrapper to download a pre-built binary, following `linutil` as a guide.

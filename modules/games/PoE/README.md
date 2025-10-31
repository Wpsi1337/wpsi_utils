# Path of Exile Helpers

This module ships local copies of the `zone_changer` and `graph_tracker` projects under `tools/`. Running the preflight/install action will sync them into your home directory (defaults: `~/zone_changer` and `~/graph_tracker`) so you can use or modify them outside the repo. Set `ZONE_CHANGER_DIR` and `GRAPH_TRACKER_DIR` to change the destination, and export `POE_SYNC_FORCE=1` to overwrite existing installs.

Launchers are provided so you can trigger each helper directly from the toolbox once they are installed.

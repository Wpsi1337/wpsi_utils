# Path of Exile Helpers

This module ships local copies of the `zone_changer` and `graph_tracker` projects under `tools/`. Each time you invoke a helper, the scripts first call `verify-prereqs.sh` to ensure the projects exist on disk. By default they are synced to `~/zone_changer` and `~/graph_tracker`; set `ZONE_CHANGER_DIR` / `GRAPH_TRACKER_DIR` to relocate them. Use `POE_SYNC_FORCE=1` if you want to overwrite an existing installation.

After the sync step, the helper launches immediately so the toolbox acts as a one-click installer/runner for your PoE utilities.

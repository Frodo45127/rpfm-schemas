# rpfm-schemas

Schemas, patches and animation ID lookups used by [Rusted PackFile Manager][rpfm] (and any tool built on top of it) to decode Total War DB tables.

[rpfm]: https://github.com/Frodo45127/rpfm

## Contents

| File                     | Purpose                                                                                                |
|--------------------------|--------------------------------------------------------------------------------------------------------|
| `schema_<game>.ron`      | Versioned definitions for every DB table and Loc-related layout the game uses. One file per game.     |
| `patches.ron`            | Runtime overlay applied on top of the base schemas: per-field display hints, lookups, default values, etc. Edits here don't require regenerating a schema. |
| `anim_ids_<game>.csv`    | Numeric animation ID ↔ name mapping for the games that use one (Three Kingdoms, Troy, Warhammer 2 & 3). |
| `LICENSE`                | MIT license for the schemas.                                                                           |

## Contributing

Open a PR with the new or updated definition. RPFM picks up changes from this repo through its **Update → Check for Schema Updates** action.

## License

MIT — see [`LICENSE`](./LICENSE).

## Support

[![become_a_patron_button](https://user-images.githubusercontent.com/15714929/40394531-2130b9ce-5e24-11e8-91a2-bbf8e6e75d21.png)][Patreon]

[Patreon]: https://www.patreon.com/RPFM

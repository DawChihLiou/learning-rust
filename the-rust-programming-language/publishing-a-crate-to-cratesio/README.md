# Publishing A Crate to `creates.io`

#### `cargo publish`

To publish a new samver.

#### `cargo yank --vers [version]`

To prevent projects from adding the version as a new dependency. 
We can't remove versions of a crate.

Run `cargo yank --vers [version] --undo` to undo a yank.

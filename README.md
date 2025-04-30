# Untifted

The game is the most fun making you own levels! Try the default one (though very broken) for a little bit, and then try making your own.

## Making a level

The game will always load `assets/level.json`, to create your own level replace it's contents with `{ "blocks": [] }` and open the debugger. If you want to have multiple levels, you can have multiple files and chose one by renaming it `level.json`.

## Keyboard shortcuts

- `W`/`A`/`S`/`D`: Move
- `Space`: Jump
- `Esc`+`Esc`: Open debugger
	- `Right click`: Place/edit block
	- `Left click`: Delete block
	- `Tab`: Toggle fly
	- `1`/`2`/`3`: Rotate plane 90d
	- `Ctrl`+`S`: Save level
	- `Arrow up`/`Arrow down`: Move cross axis

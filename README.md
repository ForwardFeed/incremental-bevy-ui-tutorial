# Incremental bevy-ui tutorial (bevy 0.16)

This in an incremental tutorial, where I'll try to explain most of the things I can about bevy UI
in an approachable manner.
Each Increment is here to give a guiding light, to overwhelm the least possible.
You can also rip off some part of my code, and part ways from there.
Some increment may not interrest you either.
Please note it's an experimental methodology.

## if you've never run any bevy project before.

Read the whole setup from https://bevyengine.org/learn/quick-start/getting-started/setup/

## How does that tutorial works.

I'll try to implement a lot of features from the bevy 0.16 ui, each increment at a time. Effectively starting the next
increment where the previous one stopped by copying, removing previous comments and explaining new one.

The main objective will be to create a pause menu with
- keybinds

### Notes about external dependencies used here.

leafwing-input-manager will be used for controls because it is likely to be 
[upstreamed in the future](https://github.com/bevyengine/bevy/issues/435#issuecomment-1254026314)

### About the truthfullness of this tutorial.

Please note that I'm doing this as a way to learn too.
I had a lot of difficulties picking up bevy UI.
I hope making so that other people don't have
the same difficulties as I had.

If you want to contribute to the project, for example pointing a mistake of mine,
please do so in issues or a pull request.


## Increments 

To run them there's two ways.
- move to the directory and cargo run : `(cd increment-0 && cargo run )`
- cargo run --binary increment-X: `cargo run --bin increment-0` 


### Increments-0, Simple TextBox
Press Escape and have a TextBox centered saying hello world. 
Repress Escape and it will disappear for the screen.
And this forever.


### Increments-1, 3 Buttons pause menu.
Spawn a list of 3 buttons:
- Resume: close the menu.
- Settings: Will do nothing see in a future increment.
- Quit: quit the executable.


### Increments-2, creating a sub menu for settings
When clicking on the settings, a new menu appears.
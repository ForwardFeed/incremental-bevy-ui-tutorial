# Incremental bevy-ui tutorial (bevy 0.16)

This in an incremental tutorial, where I'll try to explain most of the things I can about bevy UI in an approachable manner.

Each Increment is focused on a single objective but the next increment will start from where the previous ended, Hence why I call these increments.

You're not forced to go throught from 0 to X linearly, you will find in this readme the list of all increments and what they do.

Please note it's an experimental tutoring methodology as I have currently no experience in doing tutorials. Furthermore I have little experience with Bevy, I'm just a newbie that just picked it recently and which uses this project as an excuse to learn the game engine. I tried to stick as much to the views of the current engineers working on the project and tried not to be opionated.

Correction is welcome using issues or pull requests as I apologize in advance for my poor writing skills or lack of knowledge.

## if you've never run any bevy project before.

Read the whole setup from https://bevyengine.org/learn/quick-start/getting-started/setup/

## Notes about external dependencies used here.

[leafwing-input-manager](https://github.com/Leafwing-Studios/leafwing-input-manager) will be used for controls because it is likely to be 
[upstreamed in the future](https://github.com/bevyengine/bevy/issues/435#issuecomment-1254026314)

Actually [bevy_enhanced_input](https://github.com/projectharmonia/bevy_enhanced_input) may be the one to be upstreamed in the future, but I'm making this comment later.




## Increments 

To run them there's two ways.
- move to the directory and cargo run : `(cd increment-0 && cargo run )`
- cargo run --binary increment-X: `cargo run --bin increment-0` 


### Increments-0: *Simple TextBox*

Press Escape and have a TextBox centered saying hello world. 

Re-press Escape and it will disappear from the screen.


### Increments-1: *Buttons in the pause menu.*

Spawn a list of 3 buttons:
- Resume: close the menu.
- Settings: Will do nothing see in a future increment.
- Quit: quit the executable.


### Increments-2: *Creating a sub menu for settings*

When clicking on the settings button, a new menu appears with.
- Keybinds (empty)
- PlaceHolder (empty)
- Return (Just returns to the previous menu)


### Increments-3: *Some stylization*
 
- Mouse over / mouse click changes background & border color
- Added Box Shadows


### Increments-4: *Keyboard based navigation on the menu & InputFocus*

- Navigate the menu using WASD and activate with Enter
- Include the handling of the focus resource.

###### Gamepads not implemented as I don't have a gamepad, but it would be great if someone could improve that part.

### Increments-5: *Keyboard rebind menu*

Clicking keybinds in the settings now shows a menu where you can click on a keybind to change them.
Keyboard only.

### Increments-6: *An Exposition of UI bevy components*

The last increment, I'll add plenty of things and reorganize the code.

- An UI exposition where I'll toy with precise visual components, if possible giving some controls on it.
    - align items
    - justify text
    - box shadow

## Todo

- [ ] Dev tools
- [ ] Texture
- [ ] Sound Effect
- [x] Key Rebinding
- [ ] Keybinds Serializing, (alongside saving settings)
- [ ] Scrolling
- [x] UI exposition (still needs some working)
    - [ ] more content
- [ ] Setting the window name, (probably going to have that before or while saving data)
- [ ] Conditionnal rendering
- [ ] Spacial UIs
- [ ] Observer events like OnAdd
- [ ] Adding a theme changer

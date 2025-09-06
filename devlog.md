
It would be cool:

**To Do:**
|| to make the rustacean hop from square to square rather than teleporting
|| for there to be a pause menu
|| spice up the game
|| Non resizable windowed windows, where we add a settings menu, in which a set amount of resolution options can be selected which apply both to windowed and fullscreen
    - added option between fullscreen and borderless fullscreen

**Completed**
-----|| add f11 windowmode functionality
-----|| make high scores persist past app closure

*09.06.2025, 01:27*
#### Making the total statistics save themselves
- Using serde to write to a json file
*03:21* Task Completed

#### F11 Toggle Fullscreen
Completed so far but it doesnt resize the game, only the window
this is still not fixed

*09.06.2025, 19:58*
Have worked on this for a while, managed to make it resize the contents properly by chaning both the `UiScale` resource, and the Camera's `Projection` component

Now the problem is that the size is set for when its in windowed mode, and if we attempt to resize the window by dragging it, the app crashes.

Two things are possible to implement:
1. An event reader for window resizing that changes the UiScale and Projection in proportion to the resize
2. Non resizable windowed windows, where we add a settings menu, in which a set amount of resolution options can be selected which apply both to windowed and fullscreen
    - added option between fullscreen and borderless fullscreen

We go with option 2

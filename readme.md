# This repo isnt updated.
the most recent version is in my [chess_game](github.com/bowarc/chess_game) repo but the ui depends on my layered rendering system so i cant have it as a repo.
Maybe it'll be in my [Crate](github.com/bowarc/Crates) repo with the layered renderer.

Idk if that will ever be usable but i think ggez rly need a ui crate. (+ Vupa rly needs some ui)

I heard from a friend that it is supposed to be hard to 'do right'


Current state:  
- [ ] Element
    - [x] Element Anchor
    - [x] Magic values and magic values operations 
    - [ ] Element Types
    - [ ] Element Styles
- [ ] Events
- [ ] Callbacks ? 




I might have a problem going forward with this lib, i want to design it for my game, by my game as a rly specific way to draw images.
(sprite ids, sprite bank and render requests, InstanceArrays are only queried at render time not render request creation.)
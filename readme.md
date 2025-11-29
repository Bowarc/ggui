# User interface lib for the GGEZ game framework


Current state:
- [x] Dynamic values
- [x] Events
- [x] Basic styling
- [X] Simple widgets
- [ ] Lots of usefull widgets
  - [x] Buttons
  - [x] Texts
  - [x] Text Edits
  - [x] Graphs
  - [x] Images
  - [ ] Videos
  - [ ] ?
- [x] Image in texts (Something like LoL does)

I can't port the image optimisations I had for my games without making the lib too restrictive, so storing `ggez::graphics::Image`s is for now the solution.

There is also a good amount of easy optimisations to do (like using a hashmap to store elements)
And the example isn't updated, so it's not finished yet.



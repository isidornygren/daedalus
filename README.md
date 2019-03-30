<p align="center">
  <img src="daedalus_and_minotaur.jpg">
</p>

# Daedalus Dungeon & Maze generator

Daedalus is a tool to generate mazes and dungeons, you can set dungeon & room margins, corridor size and room sizes / relativistic amount of rooms.

### Minimal example
```
extern crate daedalus;
use daedalus::Generator;

fn main() {
    let map = Generator::new().generate();
}
```

### Map gallery
![1](example_images/1.bmp)![2](example_images/2.bmp)![3](example_images/3.bmp)![4](example_images/4.bmp)

### TODO _(in order of importance)_
- Remove deadends from corridors.
- Add room shapes.
- Expand map shapes to encapsulate corridors.
- Add horisontal & vertical symmetry.
- Clean up.
- Add minotaur.
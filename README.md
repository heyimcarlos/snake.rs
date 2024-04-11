# Snake.rs | Snake Game Written with Bevy

### What's snake?
- The player (snake) moves in a 2d map (4 directions) in search of food (randomly spawned, one at a time)
- Every time the snake eats food, the length of the snake is increased, thus making the game harder
- The game is over if a collision is detected on a any of the 4 walls of the map or the snake itself

### Thought process
    - Since the speed of the snake is contant, we won't be using acceleration.

## ECS List

### Entities
- Camera
- Player (snake)

### Components
- SnakeSegment 
- Direction
- Position

#### Todos

- [ ]  Add index to spawn entity to render some text as a debugger
- [ ] Camera plugin: `spawn_camera` system that initializes a 3d or 2d camera bundle.
- [ ] Create an enlarge snake event, move that logic outside of the apply eat food system

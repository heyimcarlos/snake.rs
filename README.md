# Snake.rs | Snake Game Written with Bevy

## What's snake?

- The player (snake) moves in a 2d map (4 directions) in search of food (randomly spawned, one at atime)

- Every time the snake eats food, the length of the snake is increased, thus making the game harder

- The game is over if a collision is detected on a any of the 4 walls of the map or the snake itself

## ECS List

### Entities

- Camera
- Player (snake)
- Food

### Components

- SnakeSegment
- SnakeHeadDirection
- Direction
- Position

#### Todos

- [ ] Add index to spawn entity to render some text as a debugger
- [x] Camera plugin: `spawn_camera` system that initializes a 3d or 2d camera bundle.
- [ ] Create an enlarge snake event, move that logic outside of the apply eat food system
- [ ] Add check for random food spawn not within the cells that the snake is occupying
- [ ] Add sound
- [ ] Fix the enlarging process (the snake's tail flickers)
- [ ] Decrease the wasm bundle size
- [x] Add game UI with `bevy_egui`
  - [x] Load images for button
  - [x] Scoreboard

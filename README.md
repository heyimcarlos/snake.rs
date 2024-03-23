# Snake.rs

### What's snake?
- The player (snake) moves in a 2d map (4 directions) in search of food (randomly spawned, one at a time)
- Every time the snake eats food, the length of the snake is increased, thus making the game harder
- The game is over if a collision is detected on a any of the 4 walls of the map

### Thought process
    - Since the speed of the snake is contant, we won't be using acceleration.

## ECS List

### Entities
- Camera (2d vector?)
- Player

### Components
- Velocity (movement)

### Systems 

#### Todos
- [ ] Asset Loader Plugin (load assets in a more organized fashion)
    - SceneAsset Resource that stores the scenes for assets, making them more modular.
    - `load_asset`system that instantiates the SceneAsset Resource (struct) with a asset_server resource 
    - When the plugin builds, it should initiate the resource on startup schedule
- [ ] Camera plugin
    - `spawn_camera` system that initializes a 3d or 2d camera bundle.
- [ ] State plugin
    - setup game state with InGame, Paused, and GameOver
    - have a system that handles pausing based on some key press
- [ ] Schedule plugin
    - setup different system set options? do we need to group system execution order between different plugins?
    - choose the different system set options to group systems (we previously used DespawnEntities, UserInput, CollisionDetection) as options
- [ ] Movement plugin
    - All movement configured through this plugin?
    - we can create a moving object bundle which handles movement for all objects (literally, lol)
- [ ] Collision detection through rapier?

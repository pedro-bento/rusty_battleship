# rusty_battleship
Battleship Game in Rust

warning: currently only working on windows OS.

# windows dependencies 
```bash
cargo install cargo-vcpkg
cargo vcpkg build
cargo build
```

# run
```bash
cargo run
```

# use

The blue button creates a server on your machine and the green button joins a server.   

![alt text](https://github.com/pedro-bento/rusty_battleship/blob/master/git_examples/first_scene.png)

Place your ships with RETURN using Q/E to rotate and W/A/S/D or UP/DOWN/LEFT/RIGHT to move. 

![alt text](https://github.com/pedro-bento/rusty_battleship/blob/master/git_examples/placement_scene.png)

Place your shot with RETURN when it is green (if it's gray that means it's your opponent turn), use W/A/S/D or UP/DOWN/LEFT/RIGHT to move.  

![alt text](https://github.com/pedro-bento/rusty_battleship/blob/master/git_examples/battle_scene.png)

Take a look at the final statistics, gree/red cells represent a player/opponent hit and blue cells represent a miss.  

![alt text](https://github.com/pedro-bento/rusty_battleship/blob/master/git_examples/stats_scene.png)

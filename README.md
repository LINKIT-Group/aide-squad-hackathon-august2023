
# RustyWASM
Build and run Rust-based WASM games.


## Run Game
```
# run default app
make run

# run myapp
make run app=myapp
```
Go to http://localhost:8080

## Update Game

0. Open the ./apps/myapp path in your editor.

1. make changes
2. write changes to disk, the app should automatically recompile
3. if the code is invalid -> fix and go back to 2.
4. if the code is succesful -> refresh browser to check result
repeat until it looks nice


## Add new Game
cd apps
mkdir newgame
cp -r myapp/Cargo.toml myapp/src myapp/public newgame/
edit newgame/Cargo.toml as follows:
```
[package]
name = "newgame"
version = "0.1.0"
edition = "2021"

[dependencies]
macroquad = "0.4"
wasm-bindgen = "0.2"
```

After this you should be able to run the game with Docker
```
make run app=newgame

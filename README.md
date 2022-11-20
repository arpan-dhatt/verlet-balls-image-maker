# Verlet Balls Image Maker
Quick and dirty program to bake an image onto 4000 balls moving around using verlet physics. Inspired by [this youtube video](https://www.youtube.com/watch?v=lS_qeBy3aQI).

## Usage
```
cargo run --release -- someimage.jpg
```
After all white balls are emitted, press enter to bake the image onto balls. Wait until all the balls are emitted again. You will notice it's wrong. Press enter again to restart the simulation. It will work this time. This bug will likely remain unfixed :).

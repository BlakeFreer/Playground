# Mario Web Server

The World's worst Mario Game :tm: in your browser!

Mario's health is represented with Rust's enum system to create a finite state machine. Use the browser links to transition through the possible states.

## Background

No Boilerplate released a video on implementing a [state machine with Rust's type system](https://www.youtube.com/watch?v=z-0-bbc80JM) which used this Mario transition table as an example.

![Mario Transition Table](mario.png)

I will combine this example with the [web server tutorial](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html) from the Rust Book. Transitions will be triggered by different requests and states will be shown in the HTML.

## Example

The server boots to the Mario state. If the user requests `/item/Mushroom` then the state will transition to Super Mario, and so on for the other states.

I will add two additional transitions: `/damage` which will send `Fire/Cape -> Super -> Mario -> Dead` and `/revive` to go from `Dead -> Mario`. I will also add another item called `Snowflake` and corresponding powerup `IceMario` which will behave similarly to the other powerups.

## Usage

```bash
$ cargo run
Serving on 127.0.0.1:7878
```

Open [127.0.0.1:7878](http://127.0.0.1:7878) in your browser and click the links to transition between Mario's states.

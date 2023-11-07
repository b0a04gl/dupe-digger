<p align="center">
  <img src="image-1.png" alt="DupeDigger Logo">
</p>

# DupeDigger - The Duplicate File Detective! 🕵️‍♀️

Hey there, mate! Welcome to DupeDigger - the Duplicate File Detective! This fella here's your go-to buddy when you've got a file mess so big, you'd need a map to find your way out. Let's dive into the madness and sort things out, shall we?

## The Story

### A Standalone Start

DupeDigger started as a simple solo act. It was like Batman in his early days, fighting crime one file at a time. No fancy gadgets, just a one-man show in a digital world full of chaos. 

## The Grand Plan

Now, we're cooking up a grand plan for DupeDigger, and it's happening in stages:

1. **Stage One: Serial Sleuth (Done)**

   - Our first version works on a single thread. It's like the "Once Upon a Time" version.
   - You can tell DupeDigger what size of files to look for, and it's going to stick to the mission.
   - Don't want to see those sneaky hidden files? No worries, just turn on the "No Hidden" mode!

2. **Stage Two: Scaling Up (In Progress)**

   - Here's where things get interesting. We're adding more threads and letting DupeDigger multitask like a pro.
   - It's like going from one detective to a whole team of sleuths. Faster, smarter, and ready for bigger gigs.

3. **Stage Three: Super Sleuth (Coming Soon)**

   - In the final stage, DupeDigger's going to be like Sherlock Holmes, but for files. Expect smarter moves, speedier results, and a friendlier user experience.
   - We're tossing in extra features to keep your files organized and your digital life as smooth as a double shot of espresso.

## Let's Get Started

### What You Need

- **Rust**: If you don't have it already, you'll need Rust and Cargo. Get the lowdown [right here](https://www.rust-lang.org/tools/install).

### How to Set Up

1. Grab this repo:

   ```shell
   git clone https://github.com/yourusername/DupeDigger.git
   ```

2. Find your way to the project folder:

   ```shell
   cd DupeDigger
   ```

3. Bake your investigative tool:

   ```shell
   cargo build --release
   ```

4. It's showtime:

   ```shell
   cargo run --release -- [options] <directories>
   ```

### How to Use It

Ready to uncover some gems? Use these tricks like a pirate:

```shell
cargo run --release -- --minsize 1024 --maxsize 524288 --verbose --roots /path/to/search /another/path
```

For all the secrets, just run:

```shell
cargo run --release -- --help
```

## The Legal Stuff

This project is open-source, and we're keeping it legit with the MIT License. All the official rules are in the [LICENSE](LICENSE) if you're into that kinda stuff.

## Credits

DupeDigger wouldn't be possible without the awesome Rust community and the libraries and tools that made it all happen. It's a teamwork thing, and we love it!

---

Stay tuned for some thrills and chills as DupeDigger goes from being a one-man show to a whole crew of digital detectives, solving your file mysteries and making your tech life a breeze!

---

In this version, we've got a serial program that helps you deal with smaller tasks. Stay tuned for future updates as we take it to the next level and make it more concurrent to handle larger challenges!
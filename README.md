# dlopen_issue

I've encountered an issue on macOS with `libloading` (which I suspect is actually a problem with `dlopen`).

To quickly summarize it: dropping and then reloading a dylib does not seem to update it if the dylib uses any of Rust's standard library.

You can see the problem by running, in this repo:

```
$ cargo run -- working # This reloads correctly
$ cargo run -- broken  # This fails to reload
```

The only difference between the two variants is that the `broken` version has a call to `print!` and the `working` version doesn't. I've also reproduced this with other standard library methods, as well as once by creating an empty `String`.

I'm not able to test this on other platforms currently, so it may be limited to macOS, but it may not be.

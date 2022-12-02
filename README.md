# yaadv

**Y**et **a**nother **Adv**ent of Code CLI utility

## Features

-   Download your AOC input files
-   ...Rust
-   ...and yep, that's it for now

## Setting up the CLI

Advent of Code session token is required for `yaadv` to work.

Session token is stored inside Browser cookies. So to get your session token, Open up the devtools (Inspect tool) of your browser, and then go to:

-   for Firefox: `Storage` tab, then under `Cookies`, copy the `Value` field of the `session` cookie.
-   for Chromium: `Application` tab, then under `Cookies`, copy the `Value` field of the `session` cookie.

Once you have it, enter the token interactively using:

```
yaadv -Ci
```

## Usage

Use `yaadv -h` to see available options.

### Custom output path format

`yaadv` exposes 2 tokens `{{day}}` and `{{year}}`, so that you can set custom output path for downloaded input files.

For eg. To download all input files of 2021 AOC inside a `./inputs` folder while having input files in the format of `day1.txt`, `day2.txt`, etc., You simply do this:

```sh
yaadv -Iy 2021 -o "./inputs/day{{day}}.txt"
```

- `-Iy 2021` evaluates to "download all 25 days input files of 2021 AOC`
- `-o` for setting custom output path
 - and `{{day}}` gets substituted with the corresponding day value.

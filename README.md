# yaadv

**Y**et **a**nother **Adv**ent of Code CLI tool

## Features

-   Download your AOC input files
-   ...Rust
-   ...and yep, that's it for now

## Setting up the CLI

A valid AOC session token is required for `yaadv` to work.

To get such a token, do the following:

1. Visit the [AOC](https://adventofcode.com) site and make sure you're logged in.
2. Open your browser's developers tools (Inspect tool).
3. Navigate to
    - for Firefox: `Storage` tab, then under `Cookies`
    - for Chromium: `Application` tab, then under `Cookies`
4. In there copy the `Value` field of the `session` entry.

Once you have it, enter the token interactively using:

```
yaadv -C
```

## Usage

Use `yaadv -h` to see all available options.

Fetch all 25 days inputs for latest AOC year: (in `./inputs` by default)

```
yaadv -I
```

Managing AOC token:

```
yaadv -C
```

Fetch input for day 3 of year 2021:

```
yaadv -Id 3 -y 2021
```

### Custom output path format

`yaadv` exposes 2 tokens `{{day}}` and `{{year}}` to users, so that you can set custom output path for downloaded input files.

For eg. To download all input files of 2021 AOC inside a `./inputs` folder while having input files in the format of `day1.txt`, `day2.txt`, etc., You simply do this:

```sh
yaadv -Iy 2021 -o "./inputs/day{{day}}.txt"
```

-   `-Iy 2021` evaluates to "download all 25 days input files of 2021 AOC`
-   `-o` for setting custom output path
-   and `{{day}}` gets substituted with the corresponding day value.

## Similar Projects

<sub>I had found out about these later :-/</sub>

Check out these projects:

-   [aocf](https://github.com/nuxeh/aocf)
-   [arrive](https://github.com/tranzystorek-io/arrive)

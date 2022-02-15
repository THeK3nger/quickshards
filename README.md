# QuickShards

This is a personal executable to quickly append logs to my Obsidian daily note. However, because nothing is really Obsidian-specific, I guess it can be used to append timestamped entries to any file.

## Usage

- Install with

```bash
cargo instal --git git@github.com:THeK3nger/quickshards.git
```

- Set the environment variable `QSHARD_DAILY_PATH` to your Daily Notes location.

- Set the environment variable `QSHARD_DAILY_FORMAT` to the format of your Daily Notes filename (default is `%Y-%m-%d` for notes such as `2022-02-15.md`).

- Add an entry with

```bash
qs Whatever I want to add to my note.
```

_(there is not a lot of customization because this is tailored to my notes, but I am open to PRs if you want)_.

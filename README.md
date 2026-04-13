# Oxide RSS Reader

Oxide is an RSS Reader made in Rust. It's pretty simple and will not do much but it's currently enough for me.

![Screenshot of Oxide](https://raw.githubusercontent.com/l0wigh/Oxide/refs/heads/master/oxide.png)

## How to use

You'll need to create a configuration file `~/.config/oxide/oxide.toml` with a feeds list.

```toml
# Optionnal
back = 'j'         # Default: Esc or h
down_key = 'k'     # Default: Arrow Down or j
up_key = 'l'       # Default: Arrow Up or k
select = 'm'       # Default: Enter or l

# Required
feeds = [
	{name = "General RSS", link = "https://www.rssweblog.com/rss-feed"},
	{name = "This week in Rust", link = "https://this-week-in-rust.org/rss.xml"},
	{name = "HackerNews Front Page", link = "https://hnrss.org/frontpage"},
]

```

After that, simply run oxide. You'll be greated with your feeds list. Select one and let oxide recover the articles. You can then select the one you want to read.

If you encounter feeds that only show partial part of the article, use your select key again to load the full content.

To quit, simply press 'q'.

## Future of Oxide

I might go all-in and create some kind of "database-based" app. For now I'm fine with this method.

## Libraries used

- `reqwest`: to get the content of the feed
- `tokio`: to make stuff async
- `toml` and `serde`: to parse the config file
- `rss`: to parse RSS stuff
- `html2text`: to create markdown from the HTML page
- `ratatui`: to create the TUI
- `tui-markdown`: to make the content more readable
- `readability-rust`: to parse the full article when needed

# Discord History Tracker Converter

A tool to convert the new format of [Discord History Tracker](https://dht.chylex.com/) in a sqlite database back to the
previous Json format compatible with the [Browser-Only](https://dht.chylex.com/browser-only/) version.

Please note that not everything is implemented such as embeds, replies, reactions, uploads...

## Usage

```shell
git clone https://github.com/Quozul/dht_converter.git
cd dht_converter
cargo run -- /path/to/archive.dht /path/to/dht.json
```

## Motivations

I made this program because I do not want to rewrite
my [Discord History Tracker Statistics Viewer](https://github.com/Quozul/DHStatsViewer) to use the SQLite database
instead of the Json file.

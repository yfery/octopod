Goal: learning Rust 

# Rusty

Command line application for managing podcast feeds, with sqlite backend. 

## Workflow

First, subscribe to a podcast feed. Here `-d` flag mark podcast as downloaded, without that flag every podcast will be downloaded

    rusty subscribe -d https://my.podcast.com/rss/feed/

You can list every podcast feed you have subscribed to. Useful for getting feed id needed for unsubscribing.

    rusty list

For setting directory where podcasts will be downloaded (default is temp directory)

    rusty download-dir ~/podcast/

For updating every feeds, listing podcasts to download and download them

    rusty update
    rusty pending
    rusty download

## Building

See `doc/building.md`

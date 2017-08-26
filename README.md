# Octopod

[![Build Status](https://travis-ci.org/yfery/octopod.svg?branch=master)](https://travis-ci.org/yfery/octopod)
[![Code Coverage](https://codecov.io/gh/yfery/octopod/branch/master/graph/badge.svg)](https://codecov.io/gh/yfery/octopod/branch/master)

Command line application for managing podcast feeds, with sqlite backend. 

## Workflow

First, subscribe to a podcast feed. Here `-d` flag mark podcast as downloaded, without that flag every podcast will be downloaded

    octopod subscribe -d https://my.podcast.com/rss/feed/

You can list every podcast feed you have subscribed to. Useful for getting feed id needed for unsubscribing.

    octopod list

For setting directory where podcasts will be downloaded (default is temp directory)

    octopod download-dir ~/podcast/

Without argument, print current download directory

    octopod download-dir

For updating every feeds, listing podcasts to download and download them

    octopod update
    octopod pending
    octopod download

## Download again a podcast

List podcast for finding podcast id 

    octopod downloaded | grep my_podcast
    octopod download 412

## Building

See `doc/building.md`

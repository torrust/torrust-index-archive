# Torrust
![Test](https://github.com/torrust/torrust/actions/workflows/test.yml/badge.svg)
![Build](https://github.com/torrust/torrust/actions/workflows/build.yml/badge.svg)

## Project Description

Torrust is a suite of client-server software for hosting online torrent indexes.
**Torrust only works with the torrust tracker, it does not work with other trackers currently.**

### Features
* [X] User authentication
* [X] E-mail verification
* [X] Torrent sharing
* [X] Torrent searching & filtering
* [X] Private tracker compatible
* [X] Content & user moderation tools (*__partly implemented__*)
* [X] Website customization (*__partly implemented__*)

### Installation
Read the [installation documentation](https://torrust.github.io/torrust-documentation/installation/) for setting up the torrent index backend/frontend + [Torrust tracker](https://github.com/torrust/torrust-tracker).

### Need

The reason behind this project was that Nautilus Cyberneering GmbH needed a solution that makes it easy for anyone with a computer and internet connection to share their files with the rest of the world.

### Approach

We will develop a BitTorrent tracker with a private and public tracker option and a web application for uploading and downloading torrents that are being tracked by the BitTorrent tracker.

### Benefit

Anyone will be able to host a (private) BitTorrent tracker and online torrent index with ease. And now they can share their digital content on their own torrent index :).

### Project Structure

- [__Backend__](https://github.com/torrust/torrust/tree/main/backend): A REST API written in Rust with the Actix web framework that acts as a backend for the web application.
- [__Frontend__](https://github.com/torrust/torrust/tree/main/frontend): A Vue application that acts as a frontend for the web application.
- [__Torrust Tracker__](https://github.com/torrust/torrust-tracker): A UDP and HTTP based torrent tracker built with Rust.
- [__Torrust Documentation__](https://github.com/torrust/torrust-documentation): A website made with MkDocs that hosts all the Torrust documentation.

### Project Roadmap

*Coming soon.*

### Credits
This project was a joint effort by [Nautilus Cyberneering GmbH](https://nautilus-cyberneering.de/) and [DUTCH BITS](https://dutchbits.nl).

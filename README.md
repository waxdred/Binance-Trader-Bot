# Binance Trader Bot
A bot that tracks the leadboard on Binance and sends updates to a Discord channel via webhook, built with Rust.

## Table of Contents
- [Introduction](#Introduction)
- [Features](#Features)
- [Requirements](#Requirements)
- [Installation](#Installation)
- [Usage](#Usage)
- [Custom](#Custom)

## Introduction
The Binance Trader Bot is a tool for keeping track of the top traders of your choose on Binance and sending updates to a Discord channel. This allows traders to stay informed about the performance of the top traders and make informed decisions about their own trading strategies.

## Features
- Tracks the leadboard on Binance in real-time.
- Sends updates to a Discord channel via webhook.
- Easy to configure.
- Built with the high-performance Rust programming language.

## Requirements
- Rust if not install follow [Install rust](https://www.rust-lang.org/tools/install)
- A Discord webhook set up for the channel you want to receive updates in.

## Installation
```
git clone https://github.com/waxdred/Binance-Trader-Bot
cd Binance-Trader-Bot
```

### configure
```
configure file ./config/config.json
```
- [Custom](#Custom)

## Usage
- Execute programme: 
```
make
```
- Command make:
```
make install
make clean
make run
make clean
make re
```


## Custom
- webhook: The URL for the Discord webhook.
- delai: The delay in seconds between send webhook.
- url: An array of URLs for the leadboard pages on Binance that you want to track.
- custom: An object that contains custom fields for the Discord webhook payload.
```
Example of custom
in custom value set at "" take the value of the trader
{
    "webhook": "entry url of your webhook",
    "delai": 2,
    "url": [
        "https://www.binance.com/en/futures-activity/leaderboard/user/um?encryptedUid=03160F2ACF9A714FC1204EC5B322AB34",
        "https://www.binance.com/en/futures-activity/leaderboard/user/um?encryptedUid=AEB91A40173730B4AC9C6FCE9891956A"],
    "custom" : {
        "title": "",
        "description": "",
        "username": "",
        "thumbnailUrl": "",
        "avatar_url": "",
        "author": "",
        "content": ""
    }
}
```

## Contributing
Contributions to this project are welcome. If you'd like to contribute, please fork the repository and make your changes. Then, open a pull request and I'll review your changes.

## License
This project is licensed under the MIT License.

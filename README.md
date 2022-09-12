# Trigobot

Trigobot is a Discord bot developed by some (LEIC-A)[https://fenix.tecnico.ulisboa.pt/cursos/leic-a] students who started studying at (Instituto Superior Técnico)[https://www.tecnico.pt] in 2017. It was later updated to match the needs of the (MEIC degree)[https://fenix.tecnico.ulisboa.pt/cursos/meic-a].

Trigobot notifies the students in the server whenever a teacher adds or edits one announcement in the Fénix (faculty's official website for evaluation and information). It does this by periodically scanning the RSS feed of each course it is configured to monitor.

For the MEIC upgrade, because there were a lot of courses and not everybody was having the same courses, the bot was improved with the capability of creating a channel and a role for each course, as well as a message in which the students could react in order to receive the role.

This bot was initially developed in Python (still available in an (old repository)[https://github.com/Trigobot-Development-Team/trigobot]) but now has been updated to the Rust language, using Serenity, to achieve better performance. It has been dockerized, meaning we don't need to have the Rust tools at hand in order to build and run it.

Trigobot is configurable through a `.env` file.


## Features

* RSS feed reader
* Feed message caching to detect updates and report them
* Live-reload configuration
* Message pinning with reaction
* Role and channel creation
* Some commands just for fun


## Configuration

### Bot preferences

The `.env.example` contains all the configuration parameters this bot supports, with the appropriate explanation

### RSS configuration

A feed can be added for monitoring using

```
$$$manage rss add <NAME> <RSS LINK>
```

If the course link is not a valid RSS link it won't be added.

Whenever a new feed is added, if there is not a matching role or a channel with the same name, they will be automatically created. We can configure where the category where the new channels will be created with:

```
$$$manage category <CATEGORY ID>
```

Batch addition of feeds is also supported with

```
$$$manage rss import <JSON>
```

This JSON has the following format:

```JSON
{
  // The name of the feed
  "COURSE NAME": {
    // The time the last message has been written (UNIX Timestamp)
    "updated": 0,

    // The link to the RSS feed
    "link": "https://example.com/rss",
  },
  ...
}
```

The feeds can be forced to update with the command

```
$$$rss
```

A feed can be removed with the command:

```Bash
$$$manage rss rm <FEED NAME>
```

When this operation is performed, neither the role or the channel get deleted, only the registration message and the entry in the internal state get removed.

A list of all the possible commands is available with the command

```
$$$help
```


## Running

Trigobot can be run in production mode either by using Docker Compose:

```Bash
docker-compose up -d --build
```

Or by using `cargo`:

```Bash
cargo run --release
```

For running in development mode we only use `cargo`:

```Bash
cargo run
```

## Future features

* Replication
* Slash commands
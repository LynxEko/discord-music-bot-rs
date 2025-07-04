# A Discord Music Bot in Rust

Discord music bot made in Rust


## compiling and running

Before anything you need a `config.toml` file it should have the following format:

```toml
[discord]
bot_token = ""
guild_id = ""

[youtube]
project_id = ""

type = ""
private_key_id = ""
private_key = ""
client_email = ""
client_id = ""
auth_uri = ""
token_uri = ""
auth_provider_x509_cert_url = ""
client_x509_cert_url = ""
universe_domain = ""
```

where discord is info about a discord bot (see Create a discord bot)

and youtube is the info about an service account (see Creating a google cloud project and service account)


### Requirements

you also need `yt-dlp` to download youtube video to mp3 file

### Create a discord bot

TODO : Finish readme

### Creating a google cloud project and service account

TODO : Finish readme

# Chronos

![Example screenshot of Chronos changing a user's nickname](https://i.ibb.co/tLKvH68/Screen-Shot-2021-12-21-at-12-19-04-PM.png)

Chronos uses a single permission to safely append users' timezones to their nicknames via a slash command. It uses
the [Serenity](https://github.com/serenity-rs/serenity) Rust library to communicate with Discord. Once part of a server,
any user can use `/tz`. Chronos
uses [ephemeral messages](https://support.discord.com/hc/en-us/articles/1500000580222-Ephemeral-Messages-FAQ) so
channels won't get clogged with bot messages.

## Commands

#### `/tz` [timezone]

Append your timezone to your nickname

#### `/clear`

Remove your timezone from your nickname

#### `/custom` [UTC offset]

Append a custom UTC offset to your nickname

## Invite

https://discord.com/oauth2/authorize?client_id=884672312010485770&scope=bot%20applications.commands&permissions=134217728

## Self-host

Use the Docker image with the required environment variables.

#### Example `docker.compose`

```yaml
version: '3'
services:
  bot:
    image: ghcr.io/venashial/chronos:main
    environment:
      - DISCORD_TOKEN=0000000000
      - APPLICATION_ID=000000000
```

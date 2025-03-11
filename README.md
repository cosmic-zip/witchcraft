![banner](docs/images/splited.png)

![banner](docs/images/lineBar.png)

[![witchcraft-cybersecurity](https://snapcraft.io/witchcraft-cybersecurity/badge.svg)](https://snapcraft.io/witchcraft-cybersecurity)
![witchcraft](https://img.shields.io/github/actions/workflow/status/cosmic-zip/witchcraft/witchcraft.yml)
![GitHub issues](https://img.shields.io/github/issues/cosmic-zip/witchcraft)
![GitHub License](https://img.shields.io/github/license/cosmic-zip/witchcraft)
![GitHub top language](https://img.shields.io/github/languages/top/cosmic-zip/witchcraft)

<p align="center">
  🎉 Your OPSEC companion. Run witchcraft help or witchcraft manual (for the complete manual) 🎉
</p>

---

# NAME

**witchcraft** - A versatile toolkit for cybersecurity.

# SYNOPSIS

`witchcraft [MODULE_NAME] [OPTION]... [FILE]... [IP]...`

# DESCRIPTION

WITCHCRAFT is a cyberdeck toolkit built for runners who dive deep into the NET. It’s your all-in-one rig for data-ghosting, ICE-breaking, and corp-system infiltration. Whether you’re running forensic traces, scraping OSINT feeds, or slicing through firewalls, WITCHCRAFT’s got your back.

# EXAMPLES

- `witchcraft map.local`
  Map all open local connections.

- `witchcraft search.meta --keyword user_name`
  Search for the `user_name` keyword across over 1000 sites.

- `witchcraft map.default --target example.com`
  Perform a default port scan on the specified target.

# INSTALLATION

The project initially includes a set of default files created using advanced data analysis techniques. Final versions are merged into the main project.

- **GitHub Installation:**
  Visit [witchcraft GitHub repository](https://github.com/cosmic-zip/witchcraft).
  Go to releases, download the latest version, unzip the file, and locate `installer.sh` and `uninstall.sh`.

    ```bash
    sudo bash installer.sh
    ```

- **Snap Package Installation:**

    ```bash
    snap install witchcraft-cybersecurity
    ```

- **Build from Source:**

    ```bash
    git clone https://github.com/cosmic-zip/witchcraft
    cd witchcraft
    sudo bash build-devel.sh
    ```

    Locate the `dist` folder, unzip the file, and use `installer.sh` and `uninstall.sh`.

    The script prompts for root access, creates a `release` folder, and places built executable inside. It also provides options for downloading archives for OSINT and wordlists required for IP lookup operations.

## Spellbook Package Includes:

- Unique Wordlists: _moth_ and _ladybug_
- Default Credentials Database
- IP Geolocation and Reputation/Score
- Social Media Pages for Evil Twin Attacks
- General Wordlists for Directories and Subdomains
- MAC Address Vendor Database
- Usernames Wordlist
- XSS Wordlist
- And more!

The spellbook has now been fully compressed and is 590MB in size. To download it,
please check the README page for the updated link. If you are using the complete
distribution package, you do not need to perform this step.

- To install the complete spellbook, simply download and copy it to: /var/spellbook

**link:** https://limewire.com/d/78693105-85b5-4e1f-b8f7-f1a14c598d6b#NS9Qgl0Lc-aofG86s6cBzWYCuV35yY7ErH6YZ4pxSIQ

# EVILPAGES

Clone pages into `/var/spellbook/evilpages` using the SingleFile extension or similar tools. Example:

```bash
witchcraft server.eviltwin --address 127.0.0.1:9000 --path foo/bar/index.html
```

[SingleFile Extension](https://addons.mozilla.org/en-US/firefox/addon/single-file/)

RC FILE
To record logs of all your interactions in a smartlog that can be used for data analisys and
automatic report generation, just run:

```console
    witchcraft witchrc
```

# FLAGS SCLF

Standard Command-Line Flags (SCLF) include:

- `account` : Arguments for account info or token.
- `address` : IPv4/IPv6 or domain name.
- `ip` : IPv4/IPv6 address.
- `device` : Virtual/physical device (e.g., HDD, SSD).
- `dns/domain` : Domain name.
- `database_name` : Name of the database.
- `data` : Input data (e.g., "some data here!").
- `file` : File location.
- `folder` : Path to a folder.
- `host` : Hostname or IP address.
- `image` : Image file location.
- `interface` : Network device.
- `keyspace_name` : Cassandra keyspace name.
- `message` : Message string.
- `output` : Output file path.
- `overwrite` : Overwrite existing files.
- `password` : Plaintext password.
- `path` : File path.
- `port` : Port number.
- `protocol` : Communication protocol.
- `recursive` : Enable recursive mode.
- `secret` : File (data) to be hidden.
- `share` : Shared resource (e.g., folder, file, printer).
- `snapshot_name` : Name of the snapshot.
- `table_name` : Database table name.
- `target` : IPv4/IPv6 or domain name.
- `timeout` : Timeout duration.
- `url` : Full URL path with http/https.
- `username` : Username setup.
- `wait` : Delay duration in seconds.
- `verbose` : Enable verbose mode.
- `wordlist` : Path to a wordlist.

# PLUGINS

Witchcraft supports extensions via static files, Rust code, and `db.json`. This file allows integration of terminal-based operations. Example:

**Custom Command in Terminal:**

```bash
mycommand --flag value --key value --some foo
```

**Entry in `db.json`:**

```json
{
    "name": "mycommand",
    "description": "My command does something cool",
    "command": "mycommand --flag @@flag --key @@some_name_for_the_key"
}
```

**Final Command in Witchcraft:**

```bash
mycommand --flag foo --some_name_for_the_key bar
```

You can assign any name to a flag. Note that flags are not positional.
Repeating a flag will not create a list of values. If a flag is repeated,
only the first occurrence will be accepted. This design covers 98% of CLI
interactions. Edge cases are not supported.

# LICENSE AND TERMS

This project is licensed under the **GNU General Public License v3.0**.
WITCHCRAFT includes **IP2Proxy® LITE** and **cinsscore®** databases.

---

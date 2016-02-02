# Overview

Dynamic aliases based on the directory you are currently in.

Ever wanted to type in `server` in whole bunch of different directories and your computer just knows what you're thinking?

Now you can!

## Installation

### OSX

```
brew tap sebglazebrook/aliases
brew install aliases
```

IMPORTANT: After install you are prompted to run a command to add aliases to your shell.
Make sure you run it :-)

### Linux

TODO

### Compile from source

TODO

## Usage

To create aliases for the current directory run:

```
aliases init
```

This creates a `.aliases` file in the current directory with an empty alias structure that looks like below:

```yaml
{
  "name": "server",
  "command": "bundle exec rails server",
  "confirmation": false,
  "confirmation_message": "something here",
  "conditional": "RAILS_ENV=development",
  "unit_test": "run some command here to test something"
}
```

Edit the file and away you go.

To see all the aliases available just type:

```
aliases
```

The `.aliases` file should be checked in to your repo, spread the love with the people you are working with.

### Global Aliases

Global aliases are created but running `aliases init` in your home directory.

Global aliases are overridden by local aliases if they exist.

## Contributing

TODO

## Future features

- passing additional args through to aliases
- Being about to actually run the unit tests :-)
- Having custom aliases i.e. .superman-aliases files etc
- Autocompletion for aliases
- clean uninstall, removing shims etc

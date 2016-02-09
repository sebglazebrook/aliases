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

### Linux

TODO

### Compile from source

TODO

## Usage

To create aliases for the current directory run:

```
aliases init
```

This creates a `.aliases` file in the current directory with a commented out alias structure that looks like below:

```yaml
# alias_name:
#   command: ./super_command.sh                         # required
#   confirm: true                                       # optional
#   confirmation_message: Are you sure you are sure??   # optional
#   conditional: /bin/true                              # optional
#   unit_test: '[ true = true ]'                        # optional
```

Edit the file and then run `aliases rehash` to make the alias available.

To list all aliases available just type:

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

- clean uninstall, removing shims etc
- add a 'delayed' or 'backout' option which takes an integer and executes the command after XX seconds
- passing additional args through to aliases
- Being about to actually run the unit tests :-)
- Having custom aliases i.e. .superman-aliases files etc
- Autocompletion for aliases

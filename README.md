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
#   confirm: true                                       # optional - You will be asked to confirm before execution
#   confirmation_message: Are you sure you are sure??   # optional - If confirm is set to true then you this is your confirmation message
#   conditional: /bin/true                              # optional - A bash command that needs to be successful for the alias to run
#   backout_seconds: 3                                  # optional - Give's you a backout option (ctrl + c) before the alias is executed
#   unit_test: '[ true = true ]'                        # optional - A bash command that tells whether the alias is doing what you want
```

Edit the file and then run `aliases rehash` to make the alias available.

Here's an example of some aliases:

```yaml
l:
  command: ls
gc:
  command: git commit
deploy_production:
  command: bundle exec cap production deploy
  backout_seconds: 3
  conditional: [ `git rev-parse --abbrev-ref HEAD` == "master" ]
deploy_staging:
  command: bundle exec cap staging deploy
```

To list all aliases available just type:

```
aliases
```

The `.aliases` file should be checked in to your repo, spread the love with the people you are working with.

### Global Aliases

Global aliases are created but running `aliases init` in your home directory.

Global aliases are overridden by local aliases if they exist.

## Contributing

Do the normal things, fork the code and make a PR.

## Future features

- Sort aliases lists better and make it more obvious which ones are local
- Allow better filtering when listing aliases
- Allow parent aliases that are not global??
- Colors for user interactions
- Add info in aliases file and point to repo
- Having custom aliases i.e. .superman-aliases files etc
- Being able to actually run the unit tests :-)
- Autocompletion for aliases
- clean uninstall, removing shims etc
- aliases that take params?
- provide option to not print out anything extra to stdin, for example if used when piping
- when execution an alias command the command shows up with the process name of 'alias' can it be changed to be the underlying command that is being run?

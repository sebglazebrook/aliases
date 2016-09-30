# Overview

Dynamic aliases based on the directory you are currently in.

Ever wanted to type something like `server` in whole bunch of different directories and your computer just knows what you're thinking?

Now you can!

## Why you want this

Already know you want this? Skip to [Installation](#installation).

Bash aliases are cool but limited, they are globals and have limited configurability.

One downside of standard bash aliases is that they don't take arguments, to counter this many people (myself included) do thing like create bash functions like this:

```
function ll() {
  ls -la "$@"
}
```

This downside to this is it's hard to tell where these functions are coming from, you can't just type `which ll` and find the function.

You also end up writing a whole lot of functions that are really similar, plus none of these are dynamic or contextual

So I created `aliases` to make my life easier and more fun.

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
#   quiet: false                                        # optional - default 'false', when set to false evaluated command is printed to stderr before running
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

To list all aliases available type:

```
aliases
```

The `.aliases` file should be checked in to your repo, spread the alias love with the people you are working with.

### Global Aliases

Global aliases are created but running `aliases init` in your home directory.

Global aliases are overridden by local aliases if they exist.

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

## Features in development

To test these either compile from source or `brew install aliases --devel`

Positional arguments are working:

Example:

```yaml
vim-replace:
  command : ag -l "$0" | xargs -o vim -c "bufdo %s!$0!$1!gc"
  enable_positional_arguments: true
``````

The above alias allows you to do the following:

```
vim-replace old_text new_text
```

This replaces $0 $1 etc with the arguments you send to your alias.

This currently assumes that your position keys are continuous. For example if you have $0 $1 $5 then it will not work.

## Contributing

Do the normal things, fork the code and make a PR.


## Bugs to fix

- is user's config is out of wack, like they are missing a key, it blows up
- Being able to actually run the unit tests :-)


## Small improvements to come

- Sort aliases lists better and make it more obvious which ones are local
- Colors for user interactions
- Use a thread pool when running rehash command to avoid too many threads


## Possible future features

- Having custom aliases i.e. .superman-aliases files etc
- add crud features for aliases via command line so you don't have to edit the yaml file directly
- Autocompletion for aliases
- Allow parent aliases that are not global??
- clean uninstall, removing shims etc

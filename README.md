# Overview

Dynamic aliases based on the directory you are currently in.

Ever wanted to type something like `server` in whole bunch of different directories and your computer just knows what you're thinking?

Now you can!

## Why you want this

Already know why you want this? Jump to [Installation](#installation) section.

Bash aliases are cool but limited, they are globals and have limited configurability.

One downside of standard bash aliases is that they don't take arguments, to counter this many people (myself included) do things like create bash functions like this:

```
function ll {
  ls -la "$@"
}
```

The downside to above function is that it's hard to tell where these functions are coming from, you can't just type `which ll` and find the function.

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

Here's an example of some of the aliases:

```yaml
l:
  command: ls
gc:
  command: git commit
deploy_production:
  command: bundle exec cap production deploy
  backout_seconds: 3
  conditional: test git rev-parse --abbrev-ref HEAD` == "master"
deploy_staging:
  command: bundle exec cap staging deploy
```

To list all aliases available type:

```
aliases
```

The `.aliases` file should be checked in to your repo, spread the alias love with the people you are working with.

### Global Aliases

Global aliases are created by running `aliases init` in your home directory.

These aliases are overridden by local aliases if they exist.


## Installation

### macOS

```
brew install sebglazebrook/aliases/aliases
```

### Linux

There are some debian packages in the released directory.

### Compile from source

TODO

## Features in development

To test these either compile from source or `brew install aliases --devel`

### Positional arguments

Example:

```yaml
vim-replace:
  command: ag -l "$0" && vim -c "bufdo %s/$0/$1/gc" $(ag -l "$0")
  enable_positional_arguments: true
``````

The above alias allows you to do the following:

```
vim-replace old_text new_text
```

This replaces `$0`, `$1` etc with the arguments you send to your alias.

This currently assumes that your position keys are continuous. For example if you have `$0`, `$1`, `$5` then it will not work.

### User Aliases

Maybe you want to import a friends aliases or use aliases relevant to your work, aliases makes that easy:

```
# create a new aliases file for a user

aliases init --user superman
```

This will create a new empty `.aliases-superman` file in the current directory and add the user `superman` to the list of alias users.

If your repo already has a `.aliases-superman` the file will be left untouched and the user `superman` will be added to your list of alias users.

The next time you run `aliases rehash`, all `superman` aliases in all initialized directories will be updated.

User aliases are merged together to create a list of available aliases.

If there are clashing aliases the alias user prioritized highest wins.

You can see all users and their prioritization using:

```
aliases users
```

To change the prioritization order of the user, you currently need to edit the user list in the config file.

```
cat ~/.aliases_cfg
```

And to temporally bump a user to the top with env var, like when you are pairing and sharing your shell:
```
export ALIASES_USER=superman
```

You can also pull and sync user aliases with github as:
```
aliases clone sebglazebrook                             # this could pull from github.com/sebglazebrook/.aliases repo
aliases clone sebglazebrook https://some-other-address  # pulls from the given repo
aliases pull sebglazberook                              # pull and show the diff
```

## Contributing

Do the normal things, fork the code and make a PR.

We use docker containers so we can share the same development environment. Some aliases are in this repository to help you to get up to speed. Here the list:
- ``release`` - generates a new release given the version-number
- ``run-tests`` - run all the tests
- ``workspace`` - connect to docker environment (uses the alias ``docker-machine-name`` to get it)

## Bugs to fix

- Handle different process signals
- Check user's config is out of whack, like they are missing a key, it blows up
- Being able to actually run the unit tests :-)
- When listing, aliases alert the user if the dir hasn't been initialized

## Small improvements to come

- Handle when numbers args are not in order
- Add description of alias that can be seen in list view
- Sort aliases lists better and make it more obvious which ones are local
- Colors for user interactions
- Use a thread pool when running rehash command to avoid too many threads
- In list view add more data about the aliases, user etc
- Allow multi-line command that display is list view well

## Possible future features

- Clean uninstall, removing shims etc
- Allow user to set a default shell or override the default shell. Currently all aliases are hardcoded to run inside a bash shell, could be sh or zsh

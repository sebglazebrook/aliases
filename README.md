# Overview

Dynamic aliases based on the directory you are currently in.

Ever wanted to type in `server` and it just knows what you're thinking?

Now you can!

## Installation

TODO
brew?
apt-get?
windows?
compile from source?

## Usage

To create aliases for the current directory run:

```
aliases init
```

This creates a `.aliases` file in the current directory with an empty alias structure that looks like below:

```json
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

If you have want custom aliases run

```
MY_CUSTOM_ALIAS_NAME=superman

aliases init $MY_CUSTOM_ALIAS_NAME
```

This creates a new aliases file `.superman-aliases` which should not be committed and only used for your personal shit.

### Testing

Want to make sure your aliases are not out of date?

Each alias has a `unit_test` parameter which will run some arbitary code that you define to test whether the alias is still working. Just run

```
aliases test

```

### Global Aliases

Global aliases are created but running `aliases init` in your home directory.

Global aliases are overridden by local aliases if they exist.

### Autocompletion

Yes aliases autocomplete. :boom:

## Contributing

todo

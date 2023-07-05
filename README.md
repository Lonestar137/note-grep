# Introduction

NF is a dynamic note management script.  

## What problems does it solve?

You want to quickly bring up all the notes you've ever written on a topic all at once.

You want to minimize using the Internet for information searching.

You saw/wrote something down a few weeks ago, you remember some of it but not all of it and need it right now.

You want to jot bits of information down without worrying about the context.

You're showing someone something but forgot the exact command.

## Requirements 

- Bash environment(git bash on Windows probably works.)
- toml-cli, installable via Cargo or almost any system package manager.  

## How to use 

### Install 

Download the repo down somewhere on your system and then add the ./src/ to path.

### Usage 

Once the script is callable from your CLI, you can run `nf "topic"` and it will search your notes directory for relevant bits from your notes.
NOTE: You must have some notes in the directory otherwise you will get nothing.  And, you should be segmenting your notes into 'blocks' with the delimiter `---`.  How you do so is completely up to you.  The script just looks for a line that begins and ends with `---`.

### Example Useful aliases

It's also good to add something similar to the below to your shells config file so that you can quickly get to your notes.

``` bash
NOTEDIR=/home/${USER}/Documents/TIL/
alias til="ranger ${NOTEDIR}"
alias ntil='FILENAME=$(date +%m-%d-%y) && if [ ! -s "${NOTEDIR}${FILENAME}.md" ]; then echo "$
{FILENAME}:
---" >> "${NOTEDIR}${FILENAME}.md"; fi && $EDITOR "${NOTEDIR}${FILENAME}.md"'

```

`til` will open the notes directory in ranger, and let me see the preview for the notes.  I can also easily delete or order them.
`ntil` will create OR open todays note based on the current date.  This provides an easy way for me to reference todays note and update it throughout the day.

Ultimately, it is your choice how you manage your notes, all that is required is that you store them in a directory and that whatever sections inside the note are delimited by '---.




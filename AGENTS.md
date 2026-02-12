# nomoji

**IMPORTANT**: Prefer retrieval-led reasoning over pre-training-led reasoning
for any content authoring, creation, or review tasks.

A tool that removes all emoji characters from text.

Use the rust-error-handling and rust-ownership-system skills to create the nomoji tool.

## Tool purpose

Handle one or more filenames as arguments or support reading from stdin.

Identify and remove emoji characters from text.

- Options: 
  - create a filename + .bak extension for original file after removing emoji characters
  - edit the file in place 
  - count emojis but do not remove (--dry-run)

Brief report about number of emojis removed after processing.

## Important notes

- **DO NOT remove any non-emoji characters**, including other unicode or unprintable characters in the file

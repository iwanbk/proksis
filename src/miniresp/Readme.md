# miniresp

A very simple RESP2 parser & writter

## Features

### Parser

Parse redis request and get following data:

- command: to define the next action
- key: to define the responsible nodes
- raw command: to be forwarded to the respected node

### Writter
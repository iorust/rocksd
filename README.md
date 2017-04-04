# RocksD

A key-vaule and message queue server written in RUST using RocksDB as a backend.


## API TODO

### KV

GET key
Get the value of a key

SET key value [N if not exists|E expire|C compare and set] [milliseconds]
Set the string value of a key

SETBIT key offset value
Sets or clears the bit at offset in the string value stored at key

MGET key [key ...]
Get the values of all the given keys

MSET key value [key value ...]
Set multiple keys to multiple values

INCR key increment
Increment the integer value of a key by the given amount

### Queue

PUSH key value [value ...]
Prepend one or multiple values to a list

PUSHX key value [value ...]
Prepend a value to a list, only if the list exists

POP key [count]
Remove and get the first or more elements in a list

Read key offset [count]
Get one or more elements in a list starting at the specified offset

Remove key count
Remove one or more elements in a list from the Head

SUB pattern [channel ...]
Listen for messages published to the given channels

### Common

DEL key [key ...]
Delete one or more key

EXISTS key [key ...]
Determine if a key exists

EXPIRE key seconds
Set a key's time to live in seconds

PERSIST key
Remove the expiration from a key

RENAME key newkey
Rename a key

RENAMENX key newkey
Rename a key, only if the new key does not exist

TTL key
Get the time to live for a key

TYPE key
Determine the type stored at key

UNLINK key [key ...]
Delete a key asynchronously in another thread. Otherwise it is just as DEL, but non blocking.

LLEN key
Get the key's value length by bytes or queue' length

Count pattern
Scan cursor pattern

### Sys

PING [message]
Ping the server

INFO [section]
Get information and statistics about the server

QUIT
Close the connection

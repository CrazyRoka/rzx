# Stream Concepts

## Introduction

I/O on the Spectrum is based on channels and streams. The standard Spectrum has a limited range of I/O devices — keyboard, screen, and printer — each accessed through different BASIC commands (`PRINT`, `LPRINT`, `INPUT`). With the ZX Interface I (which added microdrives, RS-232, and networking), inventing new commands for every device would be impractical, so the stream abstraction becomes essential.

A **stream** is a collection of data going to or coming from a piece of hardware. A **channel** is associated with a particular piece of hardware. Streams are numbered 0 through 15.

`PRINT #s; [print-list]` writes data to stream `s`. `INPUT #s; [input-list]` reads data from stream `s`. Both work like their ordinary counterparts (`PRINT` and `INPUT`). Each stream has two components — an input stream and an output stream — so `INPUT #` can both prompt (write) and read.

Streams can even be changed mid-statement: `PRINT #3; "hello"; #6; "there"` — though this is confusing and best avoided.

## Opening and Closing

Before a stream is used it must be **opened** with:

```
OPEN #s, c
```

This associates stream `s` with the channel specified by string `c` and signals the device that it will be used. Multiple streams can be opened to the same device, but each stream can only be associated with a single channel.

`CLOSE #s` ends the association. On an unexpanded Spectrum, closing a channel that is already closed may crash the machine due to two ROM bugs.

The four standard channels are `"K"` (keyboard), `"S"` (screen), `"P"` (printer), and `"R"` (an internal channel used for the edit buffer). The `"R"` channel cannot be opened from BASIC.

Default streams on power-up:

| Stream | Channel |
|---|---|
| 0 | `"K"` |
| 1 | `"K"` |
| 2 | `"S"` |
| 3 | `"P"` |

Since `LPRINT` is equivalent to `PRINT #3`, and `PRINT` is equivalent to `PRINT #2`, existing channels can be redirected: `OPEN #2, "P"` sends all ordinary `PRINT` output to the printer.

## Device Independence

Streams enable device-independent programs. Instead of:

```
IF output = printer THEN LPRINT "Hello" ELSE PRINT "Hello"
```

You can open stream 4 to the desired device once and write:

```
PRINT #4; "Hello"
```

This is shorter and makes it trivial to add new output devices later. An existing program can be hacked by adding `OPEN #2, "P"` near the start, redirecting every `PRINT` to the printer.

## More Stream Commands

`LIST #s` sends a listing of the BASIC program to stream `s`. `LIST #3` is equivalent to `LLIST`.

`INKEY$ #s` reads a key from stream `s`. On an unexpanded Spectrum this only works with the keyboard channel. Note that `INKEY$` (without `#`) does a stand-alone key scan, whereas `INKEY$ #1` reads from the `"K"` device, which may change cursor mode or list the editing area if both shifts are pressed.

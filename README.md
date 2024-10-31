# Tuplet

Tuplet will be a powerful open-source asynchronous musical collaboration tool.

Currently, I am just working on the design, but will hopefully make an
implementation some time.

## Why?

Current musical collaboration tools are focused on working synchronously with
a select group of people. Currently there is no effective way for a huge number
of musicians to collaborate on one piece of music, with existing solutions
giving all collaborators full control over the project, meaning that
conflicting ideas can and will cause clashes and frustration.

## How?

Tuplet takes inspiration from the open-source asynchronous collaboration model,
and brings a similar approach to music-making. Rather than a disorganised
free-for-all, Tuplet allows users to collaborate by proposing changes to the
project in a git-like process.

## Underlying design

This is the bit that I haven't figured out yet. The design choices I need to
make are:

### Processing work

Where will the "job processing" work (ie rendering to MIDI/audio) be done? On a
server, or on the client-side?

* Server-side rendering may better-facilitate collaboration, especially on
  low-powered devices, as resources can be shared

* If it's done client-side, it would probably result in far more data being
  sent over the network (eg soundfonts, audio data, etc).

Where will the minor processing work (ie previewing audio, using editing script
tools) be done? On a server, or on the client-side?

* If it's implemented on the server-side, there would be far more latency.
  But less data would need to be sent to the device.

* Client-side rendering would have far less latency.

* Perhaps a hybrid model could be used, where some rendering is done on the
  server-side and streamed to the server, but changed parts are rendered and
  combined on the client. If a hybrid approach was used, a language like Rust
  would be ideal so that code could be shared (ie machine code on the server,
  and WASM on the client).

## Data storage

Where will the projects be stored?

* On the server? This may require a lot of programming to implement the
  collaboration system.

* Using 3rd-party tools? This would be very cool for integrating with tools
  like GitHub, which would make collaboration much easier.

* Locally? This would only be feasible for single-user projects.

* A combination? Honestly having multiple strategies for the data storage would
  be a good design feature for keeping things modular.

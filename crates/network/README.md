# Network
Contains the netcode for the game. The current implementation utilizes a naive setup where players
send in their inputs to the server, where all the inputs get resolved into the server gamestate,
and then gets sent back to the user (a.k.a. the "dumb terminal" scheme).

Because this scheme usually results in choppy frames and horrible input latency, it **will** eventually
get replaced with a more modern approach that utilizes client side predictions.
For now, the current implementation is just a stopgap measure to get at least *something* working.

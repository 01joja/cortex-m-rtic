# Fundamental changes

Here I will write down the fundamental changes I have made to RTIC. Not any refactoring.

## Removed context from software task

I removed context from software tasks and moved it to the dispatchers instead.

Now the dispatchers will have the context and pass it forward with messages passing instead.

Why? To make sure context is only generated in one place. That place is in hardware task. (last pass)

## Local and Shared resources

to do:
shared resources hardware pass and software pass
local resources - idle


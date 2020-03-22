# Command Buffers

![Command-Buffer](https://user-images.githubusercontent.com/11786283/77254365-9d60b680-6c86-11ea-8733-ca977df06fc5.png)

Command Buffer can be though of a set of execution commands, kept as a buffer,
and passed to Device Queue to later on pass to Driver to execute them.

It's kind of SQL transactions, where a Command Buffer has to be set to `Recording`, pass
whatever commands we want to execute and `Stop Recording`.

## Command Pool

Command Pool is something specific to Logical Device. They are not manually created.
Instead we ask the Logical Device to provide us the Command Pool.

Using the Command Pool, we then get Command Buffer, or re-use the already fetched Command
Buffer from the pool.

## What to do now

With Command Buffer present, we can `Start Recording` on the Buffer to record, all the Commands
we want to later execute on GPU.

Once all the commands are recorded, `Stop Recording` and Pass the updated Command Buffer
to the Device Queue.

That's it!!! That's what a Command Buffer is for an Application Dev.

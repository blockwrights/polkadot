window.SIDEBAR_ITEMS = {"attr":[["async_trait",""],["contextbounds",""],["orchestra",""],["subsystem",""]],"enum":[["FromOrchestra","A message type that a subsystem receives from an orchestra. It wraps signals from an orchestra and messages that are circulating between subsystems."],["OrchestraError","An error type that describes faults that may happen"],["Poll","Indicates whether a value is available or if the current task has been scheduled to receive a wakeup instead."],["PollNext","Type to tell [`SelectWithStrategy`] which stream to poll next."],["ToOrchestra","A type of messages that are sent from a [`Subsystem`] to the declared orchestra."]],"fn":[["make_packet","Create a packet from its parts."],["select","This function will attempt to pull items from both streams. Each stream will be polled in a round-robin fashion, and whenever a stream is ready to yield an item that item is yielded."],["select_message_channel_strategy","A functor to specify strategy of the channels selection in the `SubsystemIncomingMessages`"],["select_with_strategy","This function will attempt to pull items from both streams. You provide a closure to tell [`SelectWithStrategy`] which stream to poll. The closure can store state on `SelectWithStrategy` to which it will receive a `&mut` on every invocation. This allows basing the strategy on prior choices."]],"macro":[["poll","A macro which returns the result of polling a future once within the current `async` context."],["select","Polls multiple futures and streams simultaneously, executing the branch for the future that finishes first. If multiple futures are ready, one will be pseudo-randomly selected at runtime. Futures directly passed to `select!` must be `Unpin` and implement `FusedFuture`."]],"mod":[["futures","Abstractions for asynchronous programming."],["metered","Metered variant of mpsc channels to be able to extract metrics."],["tracing","A scoped, structured logging and diagnostics system."]],"struct":[["Context","The context of an asynchronous task."],["Delay","A future representing the notification that an elapsed duration has occurred."],["Duration","A `Duration` type to represent a span of time, typically used for system timeouts."],["Fuse","Future for the `fuse` method."],["FuturesUnordered","A set of futures which may complete in any order."],["MessagePacket","A wrapping type for messages."],["Pin","A pinned pointer."],["SignalsReceived","Watermark to track the received signals."],["SpawnedSubsystem","An asynchronous subsystem task.."],["SubsystemInstance","A running instance of some `Subsystem`."],["SubsystemMeterReadouts","Set of readouts of the `Meter`s of a subsystem."],["SubsystemMeters","Collection of meters related to a subsystem."],["Timeout","A future that wraps another future with a `Delay` allowing for time-limited futures."]],"trait":[["AnnotateErrorOrigin","A trait to support the origin annotation such that errors across subsystems can be easier tracked."],["Future","A future represents an asynchronous computation obtained by use of `async`."],["FutureExt","An extension trait for `Future`s that provides a variety of convenient adapters."],["MapSubsystem","A helper trait to map a subsystem to smth. else."],["Spawner","A spawner"],["StreamExt","An extension trait for `Stream`s that provides a variety of convenient combinator functions."],["Subsystem","A trait that describes the `Subsystem`s that can run on the `Orchestra`."],["SubsystemContext","A context type that is given to the [`Subsystem`] upon spawning. It can be used by [`Subsystem`] to communicate with other [`Subsystem`]s or spawn jobs."],["SubsystemSender","Sender end of a channel to interface with a subsystem."],["TimeoutExt","Extends `Future` to allow time-limited futures."]],"type":[["BoxFuture","An owned dynamically typed [`Future`] for use in cases where you can’t statically type your result or need to add some indirection."],["OrchestraResult","Alias for a result with error type `OrchestraError`."],["SubsystemIncomingMessages","Incoming messages from both the bounded and unbounded channel."]]};
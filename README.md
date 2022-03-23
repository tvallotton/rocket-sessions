# Rocket Sessions
Rocket sessions provides a trait for storing 
client tokens on the server for future validation. 


# Implementors
The trait is implemented for `redis::aio::Client`, and for 
an in-memory data structure under the feature `in-memory`. 

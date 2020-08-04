# hlc_log_enrich: Enrich Log Files with time sourced from a Hybrid Logical Clock.

# Problem
For scale and simplicity reasons, we want to be able to start programming the network as a single logical database. The network is the computer. However the computer is spread all over the globe is contested and constrainted. As we add events and other records to our databases we need to ensure records arrive and are added in order so the network (eventually) sees a consistent view of the world achived using CRDT algorithms as a mechanism to linearise the world.

However, we need a records to have a globally consistent view of time. This is where hybrid logical clocks ([HLC](https://cse.buffalo.edu/tech-reports/2014-04.pdf)) come in. They provide a simplified mechanism to represent time globally without having necessarily have an atomic clock everywhere.

With said HLC, records need to be updated prior to transmission. The purpose of this application is to do just that.









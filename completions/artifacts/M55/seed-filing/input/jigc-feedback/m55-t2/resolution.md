Fixed by the gate-speed PR (#8, `8ec4943f`, *one stdin feeder that tolerates a child exiting before it reads*). `support::child_stdin::feed` is the one stdin writer. It judges a child that exits without reading on its exit status, not on the broken pipe.

Re-driven on this build (the checkout at `982f910f`). `child_stdin_feed::` passes 2 of 2, 149 suite files call the feeder, and the only `.write_all(…).expect("write stdin")` left in the tests are the two doc comments that describe the old pattern.

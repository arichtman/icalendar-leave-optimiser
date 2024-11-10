pub mod hol_cal;
pub mod init_cal;
pub mod simp_event;

/*
To math on this I need a plain array?
Initial date generation could easily set up the array,
  but flipping existing holidays means mapping dates back into a now dumb representation.
We can either build the dumb representation *when we go to math*, OR, we build utility that allows mapping to and fro.
*/

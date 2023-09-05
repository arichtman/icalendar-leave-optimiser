# icalendar-leave-optimiser

Optimising calendar leave so you can be where you wanna be.

** WORK IN PROGRESS, DOES NOT REFLECT FINAL PRODUCT. NOT YET FIT FOR USE IN ANY WAY, SHAPE, OR FORM. ZERO WARRANTY IMPLIED, ZERO LIABILITY ASSUMED**

## Outline

Seems like every year someone puts out a calendar "hack" that shows how to get the best time off work.
Of course, there are issues with this:

- It circulates at the same time to everyone
- It's immediately stale
- It can't account for individual (or even regional) differences
- There's usually only one edition
- It's often only published annually

So why not make our own, on-demand, and customised.
While we're at it, let's just make the top _n_ options so we have a lil choice, ey?

The end aim is that you input your time frame, existing holidays, leave amount, and (optionally) biases, and it generates the top _n_ optimal leave distributions to maximimise holidays.

## Where we're at

I have Rust, and a dream.

## Developing

I recommend VSCode + Nix. Install extensions `arrterian.nix-env-selector` and `rust-lang.rust-analyzer` and you should be good to go.
If you like, VSCode + Devcontainers is also available.
`cargo run -- --help`

## Notes

- Library in use does not support vJournal entries
- Mind that some calendar sources contain nonstandard DTEND entries

## References

- https://public-holidays.dteoh.com/

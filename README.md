# icalendar-leave-optimiser

**Moved to private repository**

Optimising calendar leave so you can be where you wanna be.

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

## Todo

- Review modules, imports, structure. Potentially add back in library definition to Cargo.toml
- Allow multiple input calendars
- Enhance argument validation

## Notes

- Library in use does not support vJournal entries
- Mind that some calendar sources contain nonstandard DTEND entries

We can't assume the input leave calendar has any events on the final date.
This means we can't calculate the end intended region for leave.
The input of amount of leave also doesn't give us this information.

We can't assume the input leave calendars have any events at the _start_ of the period either.

Ergo: program inputs must include both bounding dates.

With bounding dates we can create an array that represents the dates.
The array will be initialized empty.
Then we'll populate static leave dates from the calendar.

The array however, has no concept of mapping indices back to the dates.
Options:
a) Amend the calendar object with array indices on the leave dates.
   Then iterate the Calendar, populating the indices.
b) Iterate the array, keeping track of the corrosponding date, and doing a lookup of some sort in the Calendar.
c) ???

Option A sounds lazier.

After the array is populated with static leave dates...
Options:
a) Iterate the array, scoring each date according to the chosen algorithm.
b) Iterate the count of optional leave dates, assessing the entire array for the next highest scoring option.
c) Something involving trees

Option A doesn't react to the placing of leave ongoing, which might miss opportunities.
Option B does, but is costly and may lead to the algorithm getting stuck, and just piling stuff into one block.
We can add some randomness to avoid deterministic selection but it may not be sufficient to escape local maximums.
We can add some whole-of-array scoring using a superlinear factor based on something like deviation in duration of continuous sections

Implementation note: I'd like this to be the strategy pattern, so it's easily and clearly swappable.
Mega-stretch goal is a plugin system using WASM.


I was hoping to use argument relations to narrow the api but most inputs we can work with.
Ze question is, do we want to?
I think for now we'll stick to duration only and assume today.
We can work on input date validation and argument grouping later

Argument behaviour matrix:

| start | end | duration | Behaviour
| 0 | 0 | 0 | Default to today + 365
| 0 | 0 | 1 | Default to today + duration
| 0 | 1 | 0 | Default to today until end
| 0 | 1 | 1 | Set start to end minus duration?
| 1 | 0 | 0 | Invalid? Default to start + 365?
| 1 | 0 | 1 | Set to start + duration
| 1 | 1 | 0 | Set to start + end
| 1 | 1 | 1 | If duration mismatch, error, else continue

## References

- [Australian public holiday calendars](https://public-holidays.dteoh.com/)
- [Jeremy Chone's error handling](https://youtu.be/j-VQCYP7wyw?si=GiA-L4bEw4mnG6kl)
- [Repo of JC's error handling](https://github.com/randallard/rust10x-error-handling)

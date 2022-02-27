# cal-opt

Optimising calendar leave

## Contributing

[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](CODE_OF_CONDUCT.md)

### Setup

Download a local copy of [sample iCalendar file](https://www.officeholidays.com/ics-clean/australia/queensland).

Using Ubuntu with PPA DeadSnakes

```Bash
sudo apt install -y python3.11-distutils python3.11

poetry env use $(which python3.11)
poetry install
poetry shell
pre-commit install

cal-opt sample.ics
```

When using VSCode, set your interpreter to `poetry env info --path`, this'll allow linting and IDE tools to use the Poetry environment.

If you get `int object is not callable` when running `jake` it's [known issue](https://github.com/sonatype-nexus-community/jake/issues/100), fix with `jake ddt --clear-cache`

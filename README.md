# cal-opt

Optimising calendar leave

## Contributing

### Setup

Using Ubuntu with PPA DeadSnakes

```Bash
sudo apt install -y python3.11-distutils python3.11

poetry env use $(which python3.11)
poetry install
poetry shell
pre-commit install

```

If you get `int object is not callable` when running `jake` it's [known issue](https://github.com/sonatype-nexus-community/jake/issues/100), fix with `jake ddt --clear-cache`

from cal_opt.main import cli


def main() -> None:
    """Indirection to allow for launch of the application"""
    cli(auto_envvar_prefix="CO")


if __name__ == "__main__":
    """Only launches the application if run, not on import"""
    main()

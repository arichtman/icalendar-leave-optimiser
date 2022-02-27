import logging
import os

import click
from dotenv import load_dotenv
from ics import Calendar as ics_cal


logger = logging.getLogger("cal-opt")
load_dotenv()


@click.command()
@click.argument("calendar_file", type=click.Path(exists=True, dir_okay=False), envvar="CO_CALENDAR_FILE")
@click.option("--debug", "-d", is_flag=True, help="Toggles debug level output", envvar="CO_DEBUG")
def cli(calendar_file, debug) -> None:
    """Produce optimised leave calendars"""
    if debug:
        logging.basicConfig(level=logging.DEBUG)
        logger.debug(debug)
        logger.debug(calendar_file)
        from pprint import pprint

        env = {key: val for key, val in os.environ.items() if key.startswith("CO")}
        pprint(env)

    else:
        logging.basicConfig(level=logging.INFO)

    with open(calendar_file, "r") as file_in:
        cal = ics_cal(imports=file_in.read())
        from pprint import pprint

        for event in cal.events:
            pprint(event)

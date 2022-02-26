import logging

from dotenv import load_dotenv


logger = logging.getLogger("cal-opt")
load_dotenv()


def cli() -> None:
    """Actual root function that handles the cli"""

    debug = True
    if debug:
        logging.basicConfig(level=logging.DEBUG)
        logger.debug(debug)

    else:
        logging.basicConfig(level=logging.INFO)
    pass

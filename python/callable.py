# Example from https://docs.python.org/3/library/typing.html#callable

from collections.abc import Callable


def feeder(get_next_item: Callable[[], str]) -> None:
    pass


def async_query(
    on_success: Callable[[int], None], on_error: Callable[[int, Exception], None]
) -> None:
    pass


def ellipsis_callable(f: Callable[..., None]) -> None:
    pass

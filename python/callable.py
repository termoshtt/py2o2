# Example from https://docs.python.org/3/library/typing.html#callable

from collections.abc import Callable


def feeder(get_next_item: Callable[[], str]) -> None:
    print(f"first  = {get_next_item()}")
    print(f"second = {get_next_item()}")
    print(f"third  = {get_next_item()}")


def async_query(
    on_success: Callable[[int], None], on_error: Callable[[int, Exception], None]
) -> None:
    pass


def ellipsis_callable(f: Callable[..., None]) -> None:
    pass

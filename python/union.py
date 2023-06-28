from typing import Union


def f_new(a: int | str) -> int | str:
    return a


def f_old(a: Union[int, str]) -> Union[int, str]:
    return a

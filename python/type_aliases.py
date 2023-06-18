# Examples at https://docs.python.org/3/library/typing.html#type-aliases

from collections.abc import Sequence
from typing import NewType

Vector = list[float]


def scale(scalar: float, vector: Vector) -> Vector:
    return [scalar * num for num in vector]


ConnectionOptions = dict[str, str]
Address = tuple[str, int]
Server = tuple[Address, ConnectionOptions]


def broadcast_message(message: str, servers: Sequence[Server]) -> None:
    pass


UserId = NewType("UserId", int)


def get_user_name(user_id: UserId) -> str:
    return f"ID = {user_id}"

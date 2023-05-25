def a1():
    print("Hello from a1!")


def a2(x: int):
    print(f"x = {x}")


def a3(y: str, z: float):
    print(f"y = {y}")
    print(f"z = {z}")


def a4() -> int:
    pass


def a5(x: int) -> str:
    pass


def a6() -> (int, str):
    pass


def a7(x: int) -> (int, str, float):
    pass


def a8(x: (int, str)) -> (int, str, (int, float)):
    pass

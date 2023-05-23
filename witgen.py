import inspect
import importlib
import sys


def tuple_as_named(ty: tuple) -> str:
    """
    >>> ty = (int, str)
    >>> tuple_as_named(ty)
    '(out0: s64, out1: string)'
    >>> ty = (int, (int, str))
    >>> tuple_as_named(ty)
    '(out0: s64, out1: tuple<s64, string>)'
    """
    tags = ["out%s: %s" % (i, type_as_tag_inner(t)) for i, t in enumerate(ty)]
    return "(" + ", ".join(tags) + ")"


def tuple_as_list(ty: tuple) -> str:
    """
    >>> ty = (int, str)
    >>> tuple_as_list(ty)
    'tuple<s64, string>'
    >>> ty = (int, (int, str))
    >>> tuple_as_list(ty)
    'tuple<s64, tuple<s64, string>>'
    """
    tags = [type_as_tag_inner(t) for t in ty]
    return "tuple<" + ", ".join(tags) + ">"


def type_as_tag_outer(ty) -> str:
    if type(ty) == type:
        return type_as_tag_primitive(ty)
    if type(ty) == tuple:
        return tuple_as_named(ty)


def type_as_tag_inner(ty) -> str:
    if type(ty) == type:
        return type_as_tag_primitive(ty)
    if type(ty) == tuple:
        return tuple_as_list(ty)


def type_as_tag_primitive(ty: type) -> str:
    if ty == str:
        return "string"
    if ty == int:
        return "s64"
    if ty == float:
        return "float64"
    return ""


def main() -> int:
    if len(sys.argv) <= 1:
        print(f"usage: {sys.argv[0]} <your_module.py>")
        return 1
    target = sys.argv[1].removesuffix(".py")
    module = importlib.import_module(target)

    functions = [
        name for name, attr in inspect.getmembers(module) if inspect.isfunction(attr)
    ]

    print(f"interface {target}")
    print("{")
    for name in functions:
        f = getattr(module, name).__annotations__
        if "return" in f:
            o = "-> " + type_as_tag_outer(f["return"])
        else:
            o = ""
        i = ""
        for key, ty in f.items():
            if key == "return":
                continue
            ty = type_as_tag_inner(ty)
            if i:
                i += ", "
            i += f"{key}: {ty}"
        print(f"{name}: func({i}) {o}")
    print("}")
    return 0


if __name__ == "__main__":
    sys.exit(main())

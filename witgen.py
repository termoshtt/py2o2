#!/usr/bin/env python3

import inspect
import importlib
import sys
import pathlib
import types


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


def generic_alias(ty: types.GenericAlias) -> str:
    if ty == types.GenericAlias(list, ty.__args__):
        for ty_ in ty.__args__:
            name = type_as_tag_inner(ty_)
        return f"list<{name}>"
    raise NotImplementedError("Type = %s" % type(ty))


def type_as_tag_outer(ty) -> str:
    if type(ty) == type:
        return type_as_tag_primitive(ty)
    if type(ty) == tuple:
        return tuple_as_named(ty)
    if type(ty) == types.GenericAlias:
        return generic_alias(ty)
    raise NotImplementedError("Type = %s" % type(ty))


def type_as_tag_inner(ty) -> str:
    if type(ty) == type:
        return type_as_tag_primitive(ty)
    if type(ty) == tuple:
        return tuple_as_list(ty)
    if type(ty) == types.GenericAlias:
        return generic_alias(ty)
    raise NotImplementedError("Type = %s" % type(ty))


def type_as_tag_primitive(ty: type) -> str:
    if ty == str:
        return "string"
    if ty == int:
        return "s64"
    if ty == float:
        return "float64"
    return ""


def witgen(target: str) -> str:
    """
    Load Python module named as given `target`,
    and generate WIT IDL
    """
    module = importlib.import_module(target)

    functions = [
        name for name, attr in inspect.getmembers(module) if inspect.isfunction(attr)
    ]
    target = target.replace("_", "-")

    buffer = []
    buffer.append(f"interface {target} {{")
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
        buffer.append(f"{name}: func({i}) {o}")
    buffer.append("}")
    return "\n".join(buffer)


def main() -> int:
    if len(sys.argv) <= 1:
        print(f"usage: {sys.argv[0]} <your_module.py>", file=sys.stderr)
        return 1
    path = pathlib.Path(sys.argv[1])
    if path.exists():
        # as single file module
        sys.path.append(str(path.parent))
        print(witgen(path.stem))
    else:
        # as installed module
        print(witgen(str(path)))
    return 0


if __name__ == "__main__":
    sys.exit(main())

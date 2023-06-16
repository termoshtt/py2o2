import inspect
import importlib
import pathlib
import sys
import json
import types


def type_as_tag(ty: type) -> dict:
    if ty == inspect._empty:
        return {"kind": None}
    if ty == int:
        return {"kind": "primitive", "name": "int"}
    if ty == str:
        return {"kind": "primitive", "name": "str"}
    if ty == float:
        return {"kind": "primitive", "name": "float"}
    if type(ty) == tuple:
        tags = [type_as_tag(t) for t in ty]
        return {"kind": "tuple", "tags": tags}
    if type(ty) == types.GenericAlias:
        if ty.__origin__ == list:
            return {"kind": "list", "inner": [type_as_tag(t) for t in ty.__args__]}
    raise NotImplementedError(f"Unsupported type = {ty}")


def inspect_module(target: str) -> str:
    module = importlib.import_module(target)
    interface = {"functions": {}}
    for name, attr in inspect.getmembers(module):
        if not inspect.isfunction(attr):
            continue
        sig = inspect.signature(getattr(module, name))
        interface["functions"][name] = {
            "name": name,
            "parameters": [
                {"name": name, "annotation": type_as_tag(p.annotation)}
                for name, p in sig.parameters.items()
            ],
            "return": type_as_tag(sig.return_annotation),
        }
    return json.dumps(interface, indent=4)


def main() -> int:
    if len(sys.argv) <= 1:
        print(f"usage: {sys.argv[0]} <your_module.py>", file=sys.stderr)
        return 1
    path = pathlib.Path(sys.argv[1])
    if path.exists():
        # as single file module
        sys.path.append(str(path.parent))
        print(inspect_module(path.stem))
    else:
        # as installed module
        print(inspect_module(str(path)))
    return 0


if __name__ == "__main__":
    sys.exit(main())

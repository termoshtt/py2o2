import inspect
import importlib
import pathlib
import sys
import json


def type_as_tag(ty: type) -> dict:
    if ty == int:
        return {"name": "int"}
    if ty == str:
        return {"name": "str"}
    if ty == float:
        return {"name": "float"}
    raise NotImplementedError(f"Unsupported type = {ty}")


def inspect_module(target: str) -> str:
    module = importlib.import_module(target)
    interface = {"functions": {}}
    for name, attr in inspect.getmembers(module):
        if not inspect.isfunction(attr):
            continue
        f = {"name": name, "parameters": []}
        sig = inspect.signature(getattr(module, name))
        for name, p in sig.parameters.items():
            f["parameters"].append(
                {"name": name, "annotation": type_as_tag(p.annotation)}
            )
        interface["functions"][name] = f
    return json.dumps(interface)


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

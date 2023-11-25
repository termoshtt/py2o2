import ast
import importlib.util


def stub_exec(package_name: str, ast_portion):
    g = {
        "__name__": package_name,
        "__package__": package_name,
    }
    l = {}
    return stub_exec_inner(package_name, ast_portion, g, l)


def stub_exec_inner(
    package_name: str, ast_portion, globals: dict, locals: dict
) -> dict:
    if isinstance(ast_portion, ast.Expr):
        exec(ast.unparse(ast_portion), globals, locals)
        return locals
    if isinstance(ast_portion, ast.Import):
        print("TODO")
        pass
    if isinstance(ast_portion, ast.ImportFrom):
        print("TODO")
        pass
    if isinstance(ast_portion, ast.Try):
        body = ast_portion.body
        # TODO error case
        return stub_exec_inner(str, body, globals, locals)
    raise RuntimeError(f"Unsupported type: {type(ast_portion)}")


def main() -> int:
    spec = importlib.util.find_spec("numpy")
    with open(spec.origin, "r") as f:
        code = f.read()

    module = ast.parse(code)
    print(stub_exec("numpy", module.body[0]))


if __name__ == "__main__":
    main()

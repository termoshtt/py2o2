import ast
import importlib.util
from typing import Any


def print_code(ast_portion: Any):
    code = ast.unparse(ast_portion)
    for n, line in enumerate(code.split("\n")):
        print(n, line)


class StubLoader(ast.NodeVisitor):
    def __init__(self, package_name: str):
        self.locals = {}
        self.globals = {
            "__name__": package_name,
            "__package__": package_name,
        }

    def visit_Expr(self, node: ast.Expr):
        print("Expr")
        print_code(node)
        exec(ast.unparse(node), self.globals, self.locals)

    def visit_Assign(self, node: ast.Assign):
        print("Assign")
        print_code(node)
        exec(ast.unparse(node), self.globals, self.locals)

    def visit_Try(self, node: ast.Try):
        print("Try")
        print_code(node)
        try:
            for st in node.body:
                self.visit(st)
        except Exception as e:
            for handler in node.handlers:
                ty = ast.unparse(handler.type)
                ty = eval(ty, self.globals, self.locals)
                if isinstance(e, ty):
                    for st in handler.body:
                        self.visit(st)
                    break
            else:
                raise e
        else:
            for st in node.orelse:
                self.visit(st)
        finally:
            for st in node.finalbody:
                self.visit(st)

    def visit_Import(self, node: ast.Import):
        print("Import")
        print_code(node)
        exec(ast.unparse(node), self.globals, self.locals)

    def visit_ImportFrom(self, node: ast.ImportFrom):
        print("ImportFrom")
        print_code(node)
        exec(ast.unparse(node), self.globals, self.locals)

    def visit_If(self, node: ast.If):
        print("If")
        print_code(node)
        if eval(ast.unparse(node.test), self.globals, self.locals):
            for st in node.body:
                self.visit(st)
        else:
            for st in node.orelse:
                self.visit(st)


def main() -> int:
    spec = importlib.util.find_spec("numpy")
    print(spec.origin)
    with open(spec.origin, "r") as f:
        code = f.read()

    module = ast.parse(code)

    visitor = StubLoader("numpy")
    visitor.visit(module)


if __name__ == "__main__":
    main()

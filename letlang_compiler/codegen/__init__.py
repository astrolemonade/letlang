from dataclasses import dataclass
from pathlib import Path

from jinja2 import Environment, PackageLoader, Template

from letlang_compiler.config import Config
from letlang_compiler.model import Model

from .project import ProjectMixin
from .exe import ExecutableMixin
from .literal import LiteralMixin
from .statements import StatementMixin
from .function import FunctionMixin
from .typing import TypingMixin
from .module import ModuleMixin



@dataclass
class CodeGen(
    Model,
    ProjectMixin,
    ExecutableMixin,
    LiteralMixin,
    StatementMixin,
    FunctionMixin,
    TypingMixin,
    ModuleMixin,
):
    config: Config
    target_dir: Path
    dependencies: list[str]

    def __post_init__(self):
        self._template_env = Environment(
            loader=PackageLoader("letlang_compiler"),
            autoescape=False,
        )

    def get_template(self, name: str) -> Template:
        return self._template_env.get_template(name)
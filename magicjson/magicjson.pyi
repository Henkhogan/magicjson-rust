from typing import Iterable
from enum import Enum

class JsonType(Enum):
    def __hash__(self) -> int: ...

class JsonItem(object):

    key: str | None
    value: str | None
    items: Iterable[JsonItem] | None
    value_type: JsonType
    value_custom_type: str

def load_file(path: str) -> JsonItem:
    pass
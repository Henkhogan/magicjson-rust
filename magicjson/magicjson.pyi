from typing import Iterable
from enum import Enum

class JsonType(Enum):
    Null = 0
    List = 1
    Dict = 2
    String = 3
    Int = 4
    Float = 5
    Bool = 6
    CustomType = 7


class JsonItem(object):

    key: str | None
    value: str | None
    items: Iterable[JsonItem] | None
    value_type: JsonType
    value_custom_type: str

def load_file(path: str) -> JsonItem:
    pass
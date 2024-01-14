from typing import Iterable
from enum import Enum

class JsonType(Enum):
    def __hash__(self) -> int: ...

class JsonItem(object): ...

    #key: str | None
    #value: list[bytes] | None
    #value_bool: bool | None
    #value_dict: dict[str, JsonItem] | None
    #value_int: int | None
    #value_list: list[JsonItem] | None
    #value_float: float | None
    #value_str: str | None
    #items: Iterable[JsonItem] | None
    #value_type: JsonType
    #value_custom_type: str
#
    #def get_str(self) -> str: ...

def load_file(path: str) -> JsonItem:
    pass
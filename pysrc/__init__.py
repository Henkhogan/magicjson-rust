from typing import Iterable, NamedTuple
from dataclasses import dataclass
from enum import Enum
import magicjson_rust as magicjson


class ValueType(Enum):
    Null = 0
    List = 1
    Dict = 2
    String = 3
    Int = 4
    Float = 5
    Bool = 6
    CustomType = 7


class JsonItem(NamedTuple):
    key: str | None
    value: str | None
    items: Iterable["JsonItem"] | None
    value_type: ValueType
    value_custom_type: str


@dataclass
class JsonWrapper:
    top_level_type: ValueType
    children: Iterable[JsonItem]

    def __next__(self):
        return next(self.children)


def parse_rust_item(item: JsonItem, mandatory_key: bool = False):
    '''
        mandatory_key: bool - raises KeyError when item.key is None
    '''
    if mandatory_key and item.key is None:
        raise KeyError(f'key is mandatory for item: {item}')

    if item.value_type == ValueType.Null:
        return None
    elif item.value_type == ValueType.List:
        return parse_rust_list(item.value)
    elif item.value_type == ValueType.Dict:
        return parse_rust_dict(item.value)
    elif item.value_type == ValueType.String:
        return item.value
    elif item.value_type == ValueType.Int:
        return int(item.value)
    elif item.value_type == ValueType.Float:
        return float(item.value)
    elif item.value_type == ValueType.Bool:
        return item.value == "true"
    elif item.value_type == ValueType.CustomType:
        return item.value_custom_type
    else:
        raise Exception("Invalid value type")
    

def parse_rust_list(iterable: Iterable[JsonItem]):
    return [parse_rust_item(item) for item in iterable]
 
def parse_rust_dict(iterable: Iterable[JsonItem]):
    return {item.key: parse_rust_item(item, mandatory_key=True) for item in iterable}

def parse_rust_input(input: JsonWrapper):
    if input.top_level_type == ValueType.Dict:
        return parse_rust_dict(input.children)
    if input.top_level_type == ValueType.List:
        return parse_rust_list(input.children)
    raise ValueError("Invalid input type") 
    


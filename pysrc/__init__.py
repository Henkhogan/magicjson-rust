from typing import Generator, NamedTuple
from dataclasses import dataclass
from enum import Enum

class ValueType(Enum):
    Null = 0
    List = 1
    Dict = 2
    String = 3
    Int = 4
    Float = 5
    Bool = 6
    CustomType = 7


class JsonItemWrapper(NamedTuple):
    key: str | None
    value: str
    value_type: ValueType
    value_custom_type: str


@dataclass
class JsonWrapper:
    top_level_type: ValueType
    children: Generator[JsonItemWrapper, None, None]

    def __next__(self):
        return next(self.children)


def parse_rust_item(item: JsonItemWrapper):
    if item.value_type == ValueType.Null:
        return None
    elif item.value_type == ValueType.List:
        return parse_rust_generator(item)
    elif item.value_type == ValueType.Dict:
        return parse_rust_generator(item)
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
    
def parse_rust_generator(input: Generator[JsonItemWrapper, None, None]):
    for item in input:
        yield parse_rust_item(item)

def parse_rust_input(input: Generator[JsonItemWrapper, None, None]):
    first = next(input)
    if first.value_type == ValueType.List:
        return parse_rust_generator(input)
    elif first.value_type == ValueType.Dict:
        return parse_rust_generator(input)
    else:
        raise Exception("Invalid input type") 
    


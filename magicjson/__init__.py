from .magicjson import *

from typing import Iterable


def parse_rust_item(item: JsonItem, mandatory_key: bool = False):
    '''
        mandatory_key: bool - raises KeyError when item.key is None
    '''
    if mandatory_key and item.key is None:
        raise KeyError(f'key is mandatory for item: {item}')

    if item.value_type == JsonType.Null:
        return None
    elif item.value_type == JsonType.List:
        return parse_rust_list(item.items)
    elif item.value_type == JsonType.Dict:
        return parse_rust_dict(item.items)
    elif item.value_type == JsonType.String:
        return bytearray(item.value).decode("utf-8")
    elif item.value_type == JsonType.Int:
        return int(bytearray(item.value))
    elif item.value_type == JsonType.Float:
        return float(bytearray(item.value))
    elif item.value_type == JsonType.Bool:
        return item.value == "true"
    elif item.value_type == JsonType.CustomType:
        return item.value_custom_type
    else:
        raise Exception("Invalid value type")
    

def parse_rust_list(iterable: Iterable[JsonItem]):
    return [parse_rust_item(item) for item in iterable]
 
def parse_rust_dict(iterable: Iterable[JsonItem]):
    return {item.key: parse_rust_item(item, mandatory_key=True) for item in iterable}

def parse_rust_input(input: JsonItem):
    if input.value_type == JsonType.Dict:
        return parse_rust_dict(input.items)
    if input.value_type == JsonType.List:
        return parse_rust_list(input.items)
    raise ValueError("Invalid input type") 
    

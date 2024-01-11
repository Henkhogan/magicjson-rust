import logging

logging.basicConfig(level=logging.DEBUG)
import magicjson_rust as magicjson

x = magicjson.load_file("tests/test1.json")

from pysrc import parse_rust_input, JsonWrapper, ValueType, JsonItem

DICT_ID = 0
LIST_ID = 1
STRING_ID = 2
INT_ID = 3
FLOAT_ID = 4
BOOL_ID = 5
NULL_ID = 6
CUSTOM_TYPE_ID = 7

parse_rust_input(x[0])

y = parse_rust_input(
    input = JsonWrapper(
        top_level_type=ValueType.Dict,
        children=(
            JsonItem(
                key="key1",
                value="value1",
                items=None,
                value_type=ValueType.String,
                value_custom_type="",
            ),
            JsonItem(
                key="key2",
                value="2",
                items=None,
                value_type=ValueType.Int,
                value_custom_type="",
            ),
            JsonItem(
                key="key3",
                value="3.3",
                items=None,
                value_type=ValueType.Float,
                value_custom_type="",
            ),
            JsonItem(
                key="key4",
                value="true",
                items=None,
                value_type=ValueType.Bool,
                value_custom_type="",
            ),
            JsonItem(
                key="key5",
                value="null",
                items=None,
                value_type=ValueType.Null,
                value_custom_type="",
            ),
            JsonItem(
                key="key6",
                value="",
                items=None,
                value_type=ValueType.CustomType,
                value_custom_type="datetime.datetime",
            ), 
            JsonItem(
                key="key7",
                value=None,
                items=[
                    JsonItem(
                        key=None,
                        value="1",
                        items=None,
                        value_type=ValueType.Int,
                        value_custom_type="",
                    ),
                    JsonItem(
                        key=None,
                        value="2",
                        items=None,
                        value_type=ValueType.Int,
                        value_custom_type="",
                    ),
                    JsonItem(
                        key=None,
                        value="3",
                        items=None,
                        value_type=ValueType.Int,
                        value_custom_type="",
                    ),
                ],
                value_type=ValueType.List,
                value_custom_type="",
            ),
            JsonItem(
                key="key8",
                value=None,
                items=[
                    JsonItem(
                        key="key8.1",
                        value="value1",
                        items=None,
                        value_type=ValueType.String,
                        value_custom_type="",
                    ),
                    JsonItem(
                        key="key8.2",
                        value="5",
                        items=None,
                        value_type=ValueType.Int,
                        value_custom_type="",
                    ),
                    JsonItem(
                        key="key8.3",
                        value=None,
                        items=[
                            JsonItem(
                                key="key8.3.1",
                                value="1",
                                items=None,
                                value_type=ValueType.Int,
                                value_custom_type="",
                            ),
                            JsonItem(
                                key="key8.3.2",
                                value="2",
                                items=None,
                                value_type=ValueType.Int,
                                value_custom_type="",
                            ),
                            JsonItem(
                                key="key8.3.3",
                                value="3",
                                items=None,
                                value_type=ValueType.Int,
                                value_custom_type="",
                            ),
                        ],
                        value_type=ValueType.Dict,
                        value_custom_type="",
                    ),
                ],
                value_type=ValueType.Dict,
                value_custom_type="",
            ),
        ),
    )
)


print(x)
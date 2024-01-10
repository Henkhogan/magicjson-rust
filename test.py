import logging

logging.basicConfig(level=logging.DEBUG)
import magicjson_rust as magicjson

magicjson.load_file("tests/test1.json")

from pysrc import parse_rust_input, JsonWrapper, ValueType, JsonItemWrapper

DICT_ID = 0
LIST_ID = 1
STRING_ID = 2
INT_ID = 3
FLOAT_ID = 4
BOOL_ID = 5
NULL_ID = 6
CUSTOM_TYPE_ID = 7


x = parse_rust_input(
    input = JsonWrapper(
        top_level_type=ValueType.Dict,
        children=(
            JsonItemWrapper(
                key="key1",
                value="value1",
                value_type=ValueType.String,
                value_custom_type="",
            ),
            JsonItemWrapper(
                key="key2",
                value="2",
                value_type=ValueType.Int,
                value_custom_type="",
            ),
            JsonItemWrapper(
                key="key3",
                value="3.3",
                value_type=ValueType.Float,
                value_custom_type="",
            ),
            JsonItemWrapper(
                key="key4",
                value="true",
                value_type=ValueType.Bool,
                value_custom_type="",
            ),
            JsonItemWrapper(
                key="key5",
                value="null",
                value_type=ValueType.Null,
                value_custom_type="",
            ),
            JsonItemWrapper(
                key="key6",
                value="",
                value_type=ValueType.CustomType,
                value_custom_type="datetime.datetime",
            ), 
            JsonItemWrapper(
                key="key7",
                value=[
                    JsonItemWrapper(
                        key=None,
                        value="1",
                        value_type=ValueType.Int,
                        value_custom_type="",
                    ),
                    JsonItemWrapper(
                        key=None,
                        value="2",
                        value_type=ValueType.Int,
                        value_custom_type="",
                    ),
                    JsonItemWrapper(
                        key=None,
                        value="3",
                        value_type=ValueType.Int,
                        value_custom_type="",
                    ),
                ],
                value_type=ValueType.List,
                value_custom_type="",
            ),
            JsonItemWrapper(
                key="key8",
                value=[
                    JsonItemWrapper(
                        key="key8.1",
                        value="value1",
                        value_type=ValueType.String,
                        value_custom_type="",
                    ),
                    JsonItemWrapper(
                        key="key8.2",
                        value="5",
                        value_type=ValueType.Int,
                        value_custom_type="",
                    ),
                    JsonItemWrapper(
                        key="key8.3",
                        value=[
                            JsonItemWrapper(
                                key=None,
                                value="1",
                                value_type=ValueType.Int,
                                value_custom_type="",
                            ),
                            JsonItemWrapper(
                                key=None,
                                value="2",
                                value_type=ValueType.Int,
                                value_custom_type="",
                            ),
                            JsonItemWrapper(
                                key=None,
                                value="3",
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
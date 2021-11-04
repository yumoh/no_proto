# no_proto tools

## 安装

`pip install git+https://gitee.com/yumolab/no_proto.git`

## 说明

```python
from __future__ import __annotations__
from typing import List, Optional


def ping() -> str:
    return "pong"


class Buffer:
    def finish(self) -> bytes:
        "finish edit buff and get buff bytes"
        ...

    def from_json(self, json_value: str) -> bool:
        "use json string to initialize buff"
        ...

    def to_json(self) -> str:
        "buff to json stringfy"
        ...

    def set_with_json(self, path: List[str], json_value: str) -> bool:
        ...

    def json_encode(self, path: List[str]) -> str:
        ...

    def set_string(self, path: List[str], value: str) -> bool:
        ...

    def get_string(self, path: List[str]) -> Optional[str]:
        ...

    def push_string(self, path: List[str], value: str) -> int:
        ...

    def set_int(self, path: List[str], value: int) -> bool:
        ...

    def get_int(self, path: List[str]) -> Optional[int]:
        ...

    def push_int(self, path: List[str], value: int) -> int:
        ...

    def set_float(self, path: List[str], value: float) -> bool:
        ...

    def get_float(self, path: List[str]) -> Optional[str]:
        ...

    def push_float(self, path: List[str], value: float) -> int:
        ...


class Proto:
    @staticmethod
    def from_factory_es6(schema: str) -> Proto:
        """
        scheme like:
            struct({ fields: {
            name: string(),
            age: u16({ default: 0 }),
            tags: list({ of: string() })
        }})
        """

    @staticmethod
    def from_factory_bytes(schema: bytes) -> Proto:
        """
        buf = self.export_schema_bytes()
        p = Proto.from_factory_bytes(buf)
        """
        ...

    def export_schema_bytes(self) -> bytes:
        ...

    def empty(self) -> Buffer:
        ...

    def open(self, buffer: bytes) -> Buffer:
        ...

```
## 参考
[schema定义](https://docs.rs/no_proto/0.9.60/no_proto/schema/index.html)
## 引用

- [no_proto - crates.io: Rust Package Registry](https://crates.io/crates/no_proto)

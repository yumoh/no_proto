from no_proto import Buffer,Proto

def test_base():
    schema = """
        struct({ fields: {
        name: string(),
        age: u16({ default: 0 }),
        tags: list({ of: string() })
    }})
    """
    p = Proto.from_factory_es6(schema)
    buff = p.empty()
    print("==== base operate ===")
    buff.set_string(['name'],"Billy Joel")
    buff.push_string(['tags'],"male");
    buff.push_string(['tags'],"student");
    buff.set_with_json(["age"],"""3""");
    print(buff.to_json())
    print(buff.json_encode(['tags']))
    print(buff.finish())
    print("==== from json ====")
    json_value = """{"name":"Billy Joel","age":9,"tags":["male","student"]}"""
    buff2 = p.empty()
    buff2.from_json(json_value)
    print(buff2.to_json())
    print("=== bytes ====")
    schema_bytes = p.export_schema_bytes()
    buff_bytes = buff2.finish()
    p2 = Proto.from_factory_bytes(schema_bytes)
    buff3 = p2.open(buff_bytes)
    print(buff3.to_json())

def debug():
    test_base()

if __name__ == "__main__":
    debug()

from json import loads
with open("elements.json", "r") as f:
    data = loads(f.read())

with open("src/elements.rs", "w") as f:
    string = f"use crate::parser::Element;\npub static PERIODEN_SYSTEM: [Element; {len(data)}] = ["
    for key, value in data.items():
        string += f'Element {{name: "{value["names"]["de"]}", symbol: "{key.capitalize()}", atomaremasse: {float(value["atomic_mass"]["value"])}, ordnungszahl: {value["number"]}}},'
    string += "];"
    f.write(string)

import bindings.component as c

class Component(c.Component):  # Explicitly state it implements the protocol
    def print(self, input: str) -> None:
        print(input)

    def hello(self) -> str:
        return "Hello, World!"

from rustcsv._rustcsv import wrap_and_hello, PyInstanceWrapper


class Example:
    name: str

    def __init__(self, name):
        self.name = name

    def get_name(self):
        return self.name


def test_instancewrapper():
    inst = Example("Me")
    assert wrap_and_hello(inst) == "Me"


def test_py_instance_wrapper():
    inst = Example("Monty")
    assert PyInstanceWrapper(inst).get_name() == inst.get_name()

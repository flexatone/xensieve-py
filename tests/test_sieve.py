from xensieve import Sieve


def test_sieve_a() -> None:
    s = Sieve("3@2")
    assert str(s) == "Sieve{3@2}"




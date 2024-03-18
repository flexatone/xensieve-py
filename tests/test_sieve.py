from xensieve import Sieve


def test_sieve_a() -> None:
    s = Sieve("3@2")
    assert str(s) == "Sieve{3@2}"


def test_sieve_b() -> None:
    s1 = Sieve("3@2")
    s2 = Sieve("5@1")
    s3 = s1 ^ s2
    assert str(s3) == "Sieve{3@2^5@1}"





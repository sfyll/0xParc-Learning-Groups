import random
from typing import List

def prove_cheater(m: int, x: int, s: int) -> int:
    # return s**2 % m == x
    return 0

def verify(m: int, x: int, b: int, s: int) -> bool:
    if b == 1:
        proof = prove_cheater(m, x, s**2)
    elif b == 0:
        proof = prove_cheater(m, x, x*s**2)
    else:
        raise NotImplemented("Quantum Computing not supported yet")
    return b == proof
    


def test_fake_answer() -> None:
    m: int = 2 #every integer is a quadratic residue

    x: int = 1

    s: int = 5

    b: int = random.randint(0, 1)

    result = verify(m, x, b, s)

    return result


if __name__ == "__main__":
    """Exercise 1: Quadratic nonresidue
    We hardcode the prover answer and use dummy inputs as it doesn't really matter
    It is obvious from the piece of code that this protocol observes completness,
    it also becomes obvious as we increase iteration that soudness is also respected with probability >= 1/2
    simply by looking at how the random bit is generated"""
    results: List[int] = []
    for i in range(50000):
        results.append(int(test_fake_answer()))
    rejection_rate = (len(results) - sum(results)) / len(results)
    print(f"{rejection_rate}")
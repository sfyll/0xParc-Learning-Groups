import random

def test_exercise_one_answer() -> None:
    m: int = 2 #every integer is a quadratic residue

    x: int = 1

    s: int = 5

    b: int = random.randint(0, 1)

    result = verify_exercise_one(m, x, b, s)

    return result

def prove_cheater(m: int, x: int, s: int) -> int:
    # return s**2 % m == x
    return 0

def verify_exercise_one(m: int, x: int, b: int, s: int) -> bool:
    if b == 1:
        proof = prove_cheater(m, x, s**2)
    elif b == 0:
        proof = prove_cheater(m, x, x*s**2)
    else:
        raise NotImplementedError("Quantum Computing not supported yet")
    return b == proof

def test_exercise_two_answer() -> None:
    m: int = 4 #every integer is a quadratic residue

    x: int = 1

    s: int = 5

    t: int = 3

    b: int = random.randint(0, 1)

    result = verify_exercise_one(m, x, b, s, t)

    return result

def prove_exercise_two(s: int, t: int, b: int) -> None:
    if b == 0:
        return t
    elif b == 1:
        return s * t
    else:
        raise NotImplementedError("Quantum Computing not supported yet")


def verify_exercise_one(m: int, x: int, b: int, s: int, t: int) -> bool:
    y = x * t ** 2 % m 
    if b == 0:
        proof = prove_exercise_two(s, t, b)
        return (proof ** 2 * x) % m  == y
    elif b == 1:
        proof = prove_exercise_two(s, t, b)
        return (proof ** 2) % m == y
    else:
        raise NotImplemented("Quantum Computing not supported yet")


if __name__ == "__main__":
    """Exercise 1: Quadratic nonresidue
    We hardcode the prover answer and use dummy inputs as it doesn't really matter
    It is obvious from the piece of code that this protocol observes completness,
    it also becomes obvious as we increase iteration that soudness is also respected with probability >= 1/2
    simply by looking at how the random bit is generated"""
    # results: List[int] = []
    # for i in range(50000):
    #     results.append(int(test_exercise_one_answer()))
    # rejection_rate = (len(results) - sum(results)) / len(results)
    # print(f"{rejection_rate}")
    """Exercise 2: Quadratic residue
    Again this protocol exhibits completness as assuming no cheating, the verifier solution will match the prover's in each case
    The prover can cheat and hope that the random bit will be inactivated, but that only happens with 1/2 probability, hence over time the probability of cheating and never being caught
    is of 1^-n with n the number of repetition. Hence the verifier will rejects
    with prob => 1/2 if the cheater doesn't know s
    since each iteration are independant, and because of the discrete log problem for high enough values of s it is very hard to find s. As such, it is hard for the verifier
    to know anything about inputs used
    way better answer here = https://files.boazbarak.org/crypto/lec_14_zero_knowledge.pdf"""
    result = test_exercise_two_answer()
    print(f"{result=}")

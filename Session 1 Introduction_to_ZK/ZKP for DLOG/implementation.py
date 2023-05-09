import random
from typing import Tuple

"""This example relies on clever use of mathematics in which case it is impossible for the prover to cheat repeatedly
    Indeed, the random bit sent by the verifier does the trick. if the bit is equal to 0, the prover will get lucky as
    the y term will get cancelled and the tested equality respected. Nonetheless, that won't be the case if the random bit
    is equal to one, and only the correct value of y will allow the equation to hold
    By the same token, the prover cannot solve the equation for x. Indeed, he will only be presented by either s or r at each
    iteration, hence unable to get a system of two equations allowing him to solve for x.
    Obviously, the below is not a zkSnarks, as it is use iteration"""

def dLogProof(x: int, g: int, p: int, b: int) -> Tuple[int, int]:
    '''x: random number of the provers choosing
       g: generator from which we take the modulo p
       p : a large prime number
       b: a random bit'''

    r: int = random.randint(0, p - 1)
    h: int = (g**r) % p
    s: int = (r + b * x) % (p - 1)

    return h, s


def verify(y: int, g: int, p: int, s: int, h: int, b: int) -> bool:
    '''y: residue of interest
       g: generator from which we take the modulo p
       p: a large prime number
       s: proof sent from prover -> I know x, at least officialy when b != 0
       h: inputs for verifier to cross-check'''
    
    return  (g ** s) % p == (h * (y ** b)) % p

def test() -> None:
    x: int = 17
    p: int = 31
    g: int = 3 #is A in readings
    y: int = 22 # is B in readings

    b: int = random.randint(0, 1)

    print(f"if b is 0, cheating prover got lucky ... {b=}")

    h, s = dLogProof(x, g, p, b)

    good_result = verify(y, g, p, s, h, b)
    print(f'This should be true: {good_result}')
    bad_result = verify(23, g, p, s, h, b)
    print(f'This should be false: {bad_result}')


if __name__ == "__main__":
    test()
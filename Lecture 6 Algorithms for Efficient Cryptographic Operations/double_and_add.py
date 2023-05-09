import binascii
from cryptography.hazmat.primitives.asymmetric import ec
import hashlib
from typing import Optional, List, Tuple

def binary_expansion(number: int, base: int) -> List[int] :
    binary_representation: List[int] = []
    quotient = number
    while quotient:
        binary_representation.append(quotient % base)
        quotient = quotient // base
    return binary_representation[::-1]

def double_and_add(coordinate: Tuple[int, int], number: int, elliptic_curve_coefficient: int, prime_field: int):
    bits_rep = binary_expansion(number, 2)
    result: Optional[Tuple] = None #start at infinity, i.e. 0
    for bit in bits_rep:
        result = double(result, elliptic_curve_coefficient, prime_field)
        if bit:
            result = add(result, coordinate, elliptic_curve_coefficient, prime_field)
    return result

def double(point: Tuple[int, int], elliptic_curve_coefficient: int, prime_field: int) -> Tuple[int, int]:
    if point is None:
        return None

    #get derivative
    if point[1] == 0:
        return None
    else:
        slope = ((3 * point[0] ** 2 + elliptic_curve_coefficient) * pow(2 * point[1], -1, prime_field)) % prime_field

    # Calculate the new x-coordinate
    x3 = (slope ** 2 - 2 * point[0]) % prime_field

    # Calculate the new y-coordinate
    y3 = (slope * (point[0] - x3) - point[1]) % prime_field

    return (x3, y3)

def add(coordinate_a: Tuple[int, int], coordinate_b: Tuple[int, int], elliptic_curve_coefficient: int, prime_field: int) -> Tuple[int, int]:
    # Check if either point is at infinity
    if coordinate_a is None:
        return coordinate_b
    elif coordinate_b is None:
        return coordinate_a

    # Check if the points are equal
    if coordinate_a == coordinate_b:
        return double((coordinate_a, coordinate_b), elliptic_curve_coefficient, prime_field)
    
        # Calculate the slope of the line
    if coordinate_a[0] == coordinate_b[0]:
        return None
    else:
        slope = ((coordinate_b[1] - coordinate_a[1]) * pow(coordinate_b[0] - coordinate_a[0], -1, p)) % p

    # Calculate the new x-coordinate
    x3 = (slope ** 2 - coordinate_a[0] - coordinate_b[0]) % p

    # Calculate the new y-coordinate
    y3 = (slope * (coordinate_a[0] - x3) - coordinate_a[1]) % p

    return (x3, y3)

def double_and_add_group_theory(g: ec.EllipticCurvePublicKey, n: int, curve: ec.EllipticCurve) -> ec.EllipticCurvePublicKey:
    # Initialize the result to the point at infinity
    result = curve.identity()

    # Loop through the binary representation of n, starting from the most significant bit
    for bit in bin(n)[2:]:
        # Double the result
        result = curve.double(result)

        # If the current bit is 1, add g to the result
        if bit == '1':
            result = curve.add(result, g)

    return result

if __name__ == "__main__":
    # y^2 = x^3 + 2x + 2 (mod 17)
    g = (5, 1)
    a = 2
    b = 2
    p = 17
    order = 19



    """Using Algebra"""
    # print(double_and_add(g, 7, a, p))    
    
    """Using Group Theory, below is pseudo code, can use ESDCA lib to actually run it"""
    # Create an instance of the elliptic curve
    curve = ec.SECT163K1()

    # Create an instance of the starting point
    g_public_numbers = ec.EllipticCurvePublicNumbers(g[0], g[1], curve)
    g = g_public_numbers.public_key()
    # Perform the double-and-add algorithm
    result = double_and_add(g, 7, curve)

    # Print the result
    print(result.public_numbers().x, result.public_numbers().y)

    """Main point of showing both sides is to have a grasp of how powerful can modular arithmetic be in a world constrained by computing power"""

    


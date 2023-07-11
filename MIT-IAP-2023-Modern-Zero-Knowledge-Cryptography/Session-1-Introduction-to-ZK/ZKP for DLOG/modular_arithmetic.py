import copy
from typing import List
import math


#solution to chinese reminder theorem for fix case : just to understand the inner work, can be expanded for generic purpose (ask chatgpt)
#https://www.themathdoctors.org/chinese-remainders-with-and-without-the-theorem/
def search_by_sieving_attempt(divisors: List[int], remainders: List[int]) -> int:

    assert len(divisors) == len(remainders)
    
    #check that the divisor are pairwise coprime and get higher bound, if coprime GCD = 1
    assert checkPairwiseCoPrime(divisors, len(divisors))

    index_by_terms = get_index_by_terms(divisors)

    one_solution: int = 0

    for product, index_list in index_by_terms.items():
        remainder = copy.copy(remainders)
        divisor = copy.copy(divisors)
        for index in index_list:
            remainder.pop(index)
            divisor.pop(index)
        result = solve_equation(product, divisor[0], remainder[0])
        one_solution += result * product

    return one_solution % math.prod(divisors)  

# Function to calculate GCD
def GCD(a, b):
     
    if (a == 0):
        return b
         
    return GCD(b % a, a)
 
# Function to calculate LCM
def LCM(a, b):
     
    return (a * b) // GCD(a, b)

# Function to check if aelements
# in the array are pairwise coprime
def checkPairwiseCoPrime(A, n):
     
    # Initialize variables
    prod = 1
    lcm = 1
 
    # Iterate over the array
    for i in range(n):
 
        # Calculate product of
        # array elements
        prod *= A[i]
 
        # Calculate LCM of
        # array elements
        lcm = LCM(A[i], lcm)
 
    # If the product of array elements
    # is equal to LCM of the array
    if (prod == lcm):
        return True
    else:
        return False

def get_index_by_terms(input_array) -> dict:
    # Initialize an empty dictionary to store the indices that produced each combination
    combination_indices = {}
    # Loop through the input array
    for i in range(len(input_array)):
        for j in range(i+1, len(input_array)):
            # Multiply the values at indices i and j
            product = input_array[i] * input_array[j]
            # Add the indices that produced the product to the dictionary
            if product not in combination_indices:
                combination_indices[product] = sorted([i, j], reverse=True)
    # Return the output list and the dictionary of combination indices
    return combination_indices

def solve_equation(product: int, modulo: int, remainder: int) -> int:
    for i in range(0, modulo+1):
        result = product * i % modulo
        if result == remainder:
            return i 

if __name__ == "__main__":
    #What is the smallest number which when divided by 3, 7, and 11 leaves remainders 1, 6, and 5, respectively?
    answer = search_by_sieving_attempt(
        divisors = [3, 7, 11],
        remainders = [1, 6, 5]
    )

    print(answer)

    #obviously pls use extended euclidean algo
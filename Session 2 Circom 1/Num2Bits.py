
def Num2Bits(n, inn):
    out = [0] * n
    lc1 = 0
    neg = 0 if n == 0 else 2**n - inn
    for i in range(n):
        print(f"{neg >> i=}")
        print(f"{(neg >> i) & 1=}")
        out[i] = (neg >> i) & 1
        lc1 += out[i] * 2 ** i
    
    print(f"{lc1=}, {neg=}, {out}")

if __name__ == "__main__":
    Num2Bits(3, 5)
    Num2Bits(3, -5)
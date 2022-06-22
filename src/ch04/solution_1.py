from rich import print


def tribonacci(n: int) -> int:
    if n == 0:
        return 0
    elif n == 1:
        return 0
    elif n == 2:
        return 1
    return tribonacci(n=n-1) + tribonacci(n=n-2) + tribonacci(n=n-3)


if __name__ == '__main__':
    result: int = tribonacci(n=10)
    print(result)
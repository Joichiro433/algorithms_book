from typing import List

from rich import print


def tribonacci(n: int, memo: List[int]) -> int:
    if memo[n] != -1:
        return memo[n]
    memo[n] = tribonacci(n=n-1, memo=memo) + tribonacci(n=n-2, memo=memo) + tribonacci(n=n-3, memo=memo)
    return memo[n]


if __name__ == '__main__':
    n: int = 10
    memo: List[int] = [-1] * (n+1)
    memo[0], memo[1], memo[2] = 0, 0, 1
    result: int = tribonacci(n=n, memo=memo)

    print(result)

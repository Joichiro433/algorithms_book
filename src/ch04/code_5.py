from typing import Dict, Optional

from rich import print


class Fibonacci:
    def __init__(self) -> None:
        self.memo : Dict[int, int] = {0: 0, 1: 1}
    
    def calc_fibo(self, n: int) -> int:
        value: Optional[int] = self.memo.get(n)
        if value is not None:
            return value

        self.memo[n] = self.calc_fibo(n=n-1) + self.calc_fibo(n=n-2)
        return self.memo[n]


if __name__ == '__main__':
    fibonacci: Fibonacci = Fibonacci()
    result: int = fibonacci.calc_fibo(n=35)
    print(result)

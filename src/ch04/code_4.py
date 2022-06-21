from rich import print


def fibonacci(n: int) -> int:
    if n == 0:
        return 0
    elif n == 1:
        return 1
    return fibonacci(n-1) + fibonacci(n-2)


if __name__ == '__main__':
    result: int = fibonacci(n=10)
    print(result)
    
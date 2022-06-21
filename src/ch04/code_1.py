from rich import print


def recursive_func(n: int) -> int:
    if n == 0:
        return 0
    return n + recursive_func(n=n-1)
    

if __name__ == '__main__':
    print(recursive_func(n=5))
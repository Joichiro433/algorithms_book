from rich import print


def recursive_func(n: int) -> int:
    print(f'func({n})を呼び出しました')
    if n == 0:
        return 0
    result = n + recursive_func(n=n-1)
    print(f'{n} までの和 = {result}')
    return result


if __name__ == '__main__':
    recursive_func(n=5)
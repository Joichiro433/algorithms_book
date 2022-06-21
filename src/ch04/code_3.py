from rich import print


def euclidean_algorithm(m: int, n: int) -> int:
    if n == 0:
        return m
    return euclidean_algorithm(m=n, n=m%n)


if __name__ == '__main__':
    result: int = euclidean_algorithm(m=51, n=15)
    print(result)
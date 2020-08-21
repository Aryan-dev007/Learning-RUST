def solve():
    n, k = map(int, input().split())

    res = 0

    for i in range(0, k):
        if n % 2 == 0:
            res = n + 2
            n = res
 
        else:
            j = 3
            while j <= n:
                if n % j == 0:
                    res = n + j
                    n = res
            j += 2

    print(res)

t = int(input())

while t > 0:
    solve()
    t -= 1
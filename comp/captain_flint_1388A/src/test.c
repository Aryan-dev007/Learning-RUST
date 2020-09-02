#include<stdio.h>
void solve();
int main() {
    int t;
    scanf("%d", &t);

    while (t--) {
    solve();
    }
}

void solve() {
    int n;
    scanf("%d", &n);

    if (n <= 30) {
        printf("NO\n");
    }
    else {
        printf("YES\n");

        if (n == 36 || n == 40 || n == 44) {
            printf("6 10 15 %d\n", (n - 31));
        }
        else {
            printf("6 10 14 %d\n", (n - 30));
        }
    }
}

static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[5 % 6] = 337; cells[1] += arr[5 % 6];
    arr[1 % 6] = 835; cells[3] += arr[1 % 6];
    arr[7 % 6] = 60; cells[1] += arr[7 % 6];
    arr[5 % 6] = 672; cells[7] += arr[5 % 6];
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    arr[5 % 6] = 669; cells[1] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
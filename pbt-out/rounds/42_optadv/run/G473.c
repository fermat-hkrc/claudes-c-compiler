static int f1(int *p, int *q) { *p += 105; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[1 % 6] = 846; cells[3] += arr[1 % 6];
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    arr[0 % 6] = 568; cells[5] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    sum += f1(&cells[7], &cells[0]);
    arr[1 % 6] = 920; cells[7] += arr[1 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
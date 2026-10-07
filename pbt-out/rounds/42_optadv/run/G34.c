static int f1(int *p, int *q) { *p += 661; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    cells[1] = cells[5] + 104;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[1] = cells[3] + 133;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[0 % 6] = 936; cells[0] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[7], &cells[1] }; box.v += *box.np; cells[7] = box.v; }
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 226u; }
    sum += f1(&cells[0], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 564u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
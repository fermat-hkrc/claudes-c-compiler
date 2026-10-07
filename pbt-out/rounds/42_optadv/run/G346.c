static int f1(int *p, int *q) { *p += 247; return *q * 2; }
static int f2(int *p, int *q) { *p += 27; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[4], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[2] = cells[4] + 82;
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    cells[0] = cells[6] + 820;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 692u; }
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    sum += f2(&cells[3], &cells[2]);
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 883u; }
    cells[3] = cells[1] + 455;
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 133u; }
    arr[0 % 6] = 303; cells[3] += arr[0 % 6];
    arr[7 % 6] = 47; cells[7] += arr[7 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
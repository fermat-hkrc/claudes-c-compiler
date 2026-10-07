static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    static int f1(int *p, int *q) { *p += 239; return *q * 2; }
    sum += f1(&cells[2], &cells[6]);
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    arr[2 % 6] = 815; cells[0] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[6] }; box.v += *box.np; cells[4] = box.v; }
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    arr[1 % 6] = 127; cells[5] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    static int f2(int *p, int *q) { *p += 606; return *q * 2; }
    sum += f2(&cells[3], &cells[6]);
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 972u; }
    cells[6] = cells[3] + 774;
    cells[2] = cells[1] + 380;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
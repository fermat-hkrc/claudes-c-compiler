static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[4] = cells[0] + 804;
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 605u; }
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 576u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[3 % 6] = 338; cells[2] += arr[3 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 196u; }
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[4 % 6] = 774; cells[5] += arr[4 % 6];
    cells[7] = cells[6] + 876;
    { struct { int v; int *np; } box = { cells[7], &cells[3] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    cells[0] = cells[4] + 34;
    static int f1(int *p, int *q) { *p += 825; return *q * 2; }
    sum += f1(&cells[7], &cells[2]);
    cells[0] = cells[7] + 411;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }